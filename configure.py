#!/usr/bin/env python

import argparse
import json
import os
import os.path
import platform
import re
import sys

ROOT_DIR = os.path.dirname(os.path.realpath(__file__))
SRC_DIR = os.path.join(ROOT_DIR, 'src')
BUILD_DIR = os.path.join(ROOT_DIR, 'build')

HOST_OS = platform.system();
SUPPORTED_HOST_OSS = [ 'Darwin', 'Linux' ];

SUPPORTED_TARGET_ARCHS = [ 'x86_64', 'aarch64' ];
DEFAULT_TARGET_ARCH = platform.machine() \
    if platform.machine() in SUPPORTED_TARGET_ARCHS else 'x86_64'

SUPPORTED_OPT_LEVELS = [ 0, 1, 2, 3 ]
DEFAULT_OPT_LEVEL = 3

DEFAULT_BINUTILS_INFIXES = {
    'Darwin': '-elf-', 'Linux': '-linux-gnu-'
};
DEFAULT_BINUTILS_INFIX = DEFAULT_BINUTILS_INFIXES[HOST_OS] \
    if HOST_OS in DEFAULT_BINUTILS_INFIXES else ''

parser = argparse.ArgumentParser(description='Configure Arwen OS build.')
parser.add_argument('--arch', dest='target_arch', action='store',
                    choices=SUPPORTED_TARGET_ARCHS,
                    default=DEFAULT_TARGET_ARCH,
                    help='target architecture (default: %s)'% \
                        DEFAULT_TARGET_ARCH)
parser.add_argument('--opt', dest='opt_level', action='store',
                    type=int, choices=SUPPORTED_OPT_LEVELS,
                    default=DEFAULT_OPT_LEVEL,
                    help='optimization level (default: %d)'%DEFAULT_OPT_LEVEL)
parser.add_argument('--pref', dest='binutils_prefix', action='store',
                    help='binutils prefix (default: <arch>%s)'% \
                        DEFAULT_BINUTILS_INFIX)
args = parser.parse_args()

if not args.binutils_prefix:
    args.binutils_prefix = args.target_arch + DEFAULT_BINUTILS_INFIX

if HOST_OS not in SUPPORTED_HOST_OSS:
    sys.stderr.write('error: unsupported host OS\n')
    sys.exit(1)

def which(program):
    def is_exe(fpath):
        return os.path.isfile(fpath) and os.access(fpath, os.X_OK)

    fpath, fname = os.path.split(program)
    if fpath:
        if is_exe(program):
            return program
    else:
        for path in os.environ['PATH'].split(os.pathsep):
            path = path.strip('"')
            exe_file = os.path.join(path, program)
            if is_exe(exe_file):
                return exe_file

    return None

def find_program(program):
    filename = which(program)
    if not filename:
        sys.stderr.write("error: didn't find '%s' program\n"%program)
        sys.exit(1)
    return filename

rustc = find_program('rustc')
objcopy = find_program(args.binutils_prefix+'objcopy')
ar = find_program(args.binutils_prefix+'ar')
gas = find_program(args.binutils_prefix+'as')
ld = find_program(args.binutils_prefix+'ld')

def read_build_json(filename):
    with open(filename, 'r') as f:
        return json.loads(f.read())

def find_extern_crates(filename):
    with open(filename, 'r') as f:
        return re.findall('extern\s+crate\s+(\w+)\s*;', f.read())

def build_dir(path):
    relpath = os.path.relpath(path, ROOT_DIR)
    return os.path.join(BUILD_DIR, relpath)

def prettify_target(filename):
    return os.path.relpath(filename, BUILD_DIR)

def rlib_target(name):
    return os.path.join(build_dir(SRC_DIR), name, 'lib%s.rlib'%name)

def robj_target(name):
    return os.path.join(build_dir(SRC_DIR), name, '%s.rust.o'%name)

def sobj_target(name):
    return os.path.join(build_dir(SRC_DIR), name, '%s.asm.o'%name)

def mod_target(name):
    return os.path.join(build_dir(SRC_DIR), name, '%s.mod'%name)

class Module:
    def __init__(self, name):
        self.name = name
        self.dependencies = \
            map(lambda m: rlib_target(m),
                find_extern_crates(os.path.join(self.path(), 'lib.rs')))
        self.asm_files = []
        self.rust_files =[]
        self.build_dirs = []
        self.find_all_files(self.path())

    def path(self):
        return os.path.join(SRC_DIR, self.name)

    def find_all_files(self, path):
        data = read_build_json(os.path.join(path, 'build.json'))
        if 'asmFiles' in data:
            self.asm_files += map(
                lambda f: os.path.join(path, f), data['asmFiles'])
        if 'rustFiles' in data:
            self.rust_files += map(
                lambda f: os.path.join(path, f), data['rustFiles'])
        if 'subdirs' in data:
            for d in data['subdirs']:
                if d == 'arch': d += '-' + args.target_arch
                self.find_all_files(os.path.join(path, d))
        else:
            self.build_dirs.append(build_dir(path))

    def build_dirs_target(self):
        return os.path.join(build_dir(self.path()), '.build_dirs')

    def render(self, makefile):
        makefile.write('# Module: %s\n'%self.name)
        self.render_build_dirs(makefile)
        self.render_rlib(makefile)
        self.render_robj(makefile)
        self.render_sobj(makefile)
        self.render_mod(makefile)

    def render_build_dirs(self, makefile):
        target = self.build_dirs_target()
        makefile.write('\n%s:\n'%target)
        makefile.write('\t@echo "Creating build dirs for \'%s\'"\n'%self.name)
        makefile.write('\t@mkdir -p %s\n'%(' '.join(self.build_dirs)))
        makefile.write('\t@touch %s\n'%target)

    def render_rlib(self, makefile):
        target = rlib_target(self.name)
        rlibs = ' ' + ' '.join(self.dependencies) if self.dependencies else ''
        rfiles = ' ' + ' '.join(self.rust_files) if self.rust_files else ''

        makefile.write('\n%s: %s%s%s\n'% \
            (target, self.build_dirs_target(), rlibs, rfiles))
        makefile.write('\t@echo "Creating \'%s\'"\n'%prettify_target(target))

        flags = ('--crate-type rlib --target %s-unknown-linux-gnu ' + \
            '-C opt-level=%d -C no-stack-check -Z no-landing-pads ' + \
            '--cfg arch_%s --sysroot /dev/null') % \
            (args.target_arch, args.opt_level, args.target_arch)

        lib_path = ' -L ' + ' -L '.join(
            map(lambda d: os.path.dirname(d), self.dependencies)) \
            if self.dependencies else ''

        librs = os.path.join(self.path(), 'lib.rs')
        makefile.write('\t@%s %s%s %s -o %s\n'% \
            (rustc, flags, lib_path, librs, target))

    def render_robj(self, makefile):
        source = rlib_target(self.name)
        target = robj_target(self.name)
        tmplib = os.path.join(build_dir(SRC_DIR),
            self.name, 'lib%s.a'%self.name)
        tmpobj = os.path.join(build_dir(SRC_DIR),
            self.name, 'lib%s.0.o'%self.name)

        makefile.write('\n%s: %s\n'%(target, source))
        makefile.write('\t@echo "Creating \'%s\'"\n'%prettify_target(target))
        makefile.write('\t@%s %s %s 2> /dev/null\n'% \
            (objcopy, source, tmplib))
        makefile.write('\t@cd %s && %s -x %s lib%s.0.o\n'% \
            (build_dir(self.path()), ar, tmplib, self.name))
        makefile.write('\t@mv %s %s\n'%(tmpobj, target))
        makefile.write('\t@rm %s\n'%tmplib)

    def render_sobj(self, makefile):
        if not self.asm_files: return
        source = ' '.join(self.asm_files)
        target = sobj_target(self.name)
        makefile.write('\n%s: %s %s\n'% \
            (target, self.build_dirs_target(), source))
        makefile.write('\t@echo "Creating \'%s\'"\n'%prettify_target(target))
        makefile.write('\t@%s %s -o %s\n'%(gas, source, target))

    def render_mod(self, makefile):
        source = '%s %s'% (sobj_target(self.name), robj_target(self.name)) \
            if self.asm_files else robj_target(self.name)
        target = mod_target(self.name)
        lds = os.path.join(SRC_DIR, 'kernel', 'module.lds')
        makefile.write('\n%s: %s\n'%(target, source))
        makefile.write('\t@echo "Creating \'%s\'"\n'%prettify_target(target))
        makefile.write('\t@%s -r -T %s %s -o %s\n'%(ld, lds, source, target))
        makefile.write('\n.PHONY: %s.mod\n%s.mod: %s\n'% \
            (self.name, self.name, target))

data = read_build_json(os.path.join(SRC_DIR, 'build.json'))
kmodules = map(lambda n: Module(n), data['kernelModules'])
kernel_target = os.path.join(build_dir(SRC_DIR), 'kernel', 'arwen.ki')

with open(os.path.join(ROOT_DIR, 'Makefile'), 'w') as f:
    f.write('# Generated by configure.py, do not modify\n')
    f.write('\n.PHONY: all\nall: arwen.ki\n')
    f.write('\n.PHONY: clean\nclean:\n')
    f.write('\t@echo "Removing build dirs"\n')
    f.write('\t@rm -r %s\n'%BUILD_DIR)

    for m in kmodules:
        f.write('\n')
        m.render(f)

    source = ' '.join(map(lambda m: mod_target(m.name), kmodules))
    lds = os.path.join(SRC_DIR, 'kernel',
        'arch-'+args.target_arch, 'kernel.lds')

    f.write('\n# Kernel\n')
    f.write('\n%s: %s\n'%(kernel_target, source))
    f.write('\t@echo "Creating \'%s\'"\n'%prettify_target(kernel_target))
    f.write('\t@%s -T %s %s -o %s\n'%(ld, lds, source, kernel_target))
    f.write('\n.PHONY: arwen.ki\narwen.ki: %s\n'%kernel_target)
