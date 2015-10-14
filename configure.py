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

SUPPORTED_ARCHS = [ 'x86_64', 'aarch64' ];
DEFAULT_ARCH = 'x86_64'

SUPPORTED_LEVELS = [ 0, 1, 2, 3 ]
DEFAULT_LEVEL = 3

DEFAULT_INFIX = '-elf-'

parser = argparse.ArgumentParser(description='Arwen OS build configurer.')
parser.add_argument('--arch', dest='arch', action='store',
                    choices=SUPPORTED_ARCHS, default=DEFAULT_ARCH,
                    help='target CPU architecture (default: %s)'%DEFAULT_ARCH)
parser.add_argument('--level', dest='level', action='store',
                    type=int, choices=SUPPORTED_LEVELS, default=DEFAULT_LEVEL,
                    help='optimization level (default: %d)'%DEFAULT_LEVEL)
parser.add_argument('--prefix', dest='prefix', action='store',
                    help='binutils prefix (default: <arch>%s)'% \
                        DEFAULT_INFIX)
args = parser.parse_args()

if not args.prefix:
    args.prefix = args.arch + DEFAULT_INFIX

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
objcopy = find_program(args.prefix+'objcopy')
ar = find_program(args.prefix+'ar')
gas = find_program(args.prefix+'as')
ld = find_program(args.prefix+'ld')
qemu = find_program('qemu-system-'+args.arch)

def read_build_json(filename):
    try:
        with open(filename, 'r') as f:
            return json.loads(f.read())
    except IOError:
        return {}

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

def render(makefile, str):
    makefile.write(str.replace(ROOT_DIR, '$(ROOT)'))

class Module:
    def __init__(self, name):
        self.name = name
        self.asm_files = []
        self.rust_files =[]
        self.dependencies = []
        self.build_dirs = []
        self.rust_args = []
        self.scan_dirs(self.path())
        self.dependencies = list(set(self.dependencies))

    def path(self):
        return os.path.join(SRC_DIR, self.name)

    def scan_dirs(self, path):
        data = read_build_json(os.path.join(path, 'build.json'))
        if 'asmFiles' in data:
            self.asm_files += map(
                lambda f: os.path.join(path, f), data['asmFiles'])
        if 'rustFiles' in data:
            self.rust_files += map(
                lambda f: os.path.join(path, f), data['rustFiles'])
        if 'dependencies' in data:
            self.dependencies += map(
                lambda m: rlib_target(m), data['dependencies'])
        if 'rustArgs' in data:
            self.rust_args += data['rustArgs']
        if 'subdirs' in data:
            for d in data['subdirs']:
                if d == 'arch': d += '-' + args.arch
                self.scan_dirs(os.path.join(path, d))
        else:
            self.build_dirs.append(build_dir(path))

    def build_dirs_target(self):
        return os.path.join(build_dir(self.path()), '.build_dirs')

    def render(self, makefile):
        render(makefile, '# Module: %s\n'%self.name)
        self.render_build_dirs(makefile)
        self.render_rlib(makefile)
        self.render_robj(makefile)
        self.render_sobj(makefile)
        self.render_mod(makefile)

    def render_build_dirs(self, makefile):
        target = self.build_dirs_target()
        render(makefile, '\n%s:\n'%target)
        render(makefile, '\t@echo Creating build dirs for \'%s\'\n'%self.name)
        render(makefile, '\t@mkdir -p %s\n'%(' '.join(self.build_dirs)))
        render(makefile, '\t@touch %s\n'%target)

    def render_rlib(self, makefile):
        target = rlib_target(self.name)
        rlibs = ' ' + ' '.join(self.dependencies) if self.dependencies else ''
        rfiles = ' ' + ' '.join(self.rust_files) if self.rust_files else ''

        render(makefile, '\n%s: %s%s%s\n'% \
            (target, self.build_dirs_target(), rlibs, rfiles))
        render(makefile, '\t@echo Creating \'%s\'\n'%prettify_target(target))

        flags = ('--crate-type rlib --target %s-unknown-linux-gnu ' + \
            '-C opt-level=%d -C no-stack-check -Z no-landing-pads ' + \
            '--cfg arch_%s --sysroot /dev/null') % \
            (args.arch, args.level, args.arch)

        lib_path = ' -L ' + ' -L '.join(
            map(lambda d: os.path.dirname(d), self.dependencies)) \
            if self.dependencies else ''

        extra_args = ' ' + ' '.join(self.rust_args) if self.rust_args else ''

        librs = os.path.join(self.path(), 'lib.rs')
        render(makefile, '\t@%s %s%s%s %s -o %s\n'% \
            (rustc, flags, lib_path, extra_args, librs, target))

    def render_robj(self, makefile):
        source = rlib_target(self.name)
        target = robj_target(self.name)
        tmplib = os.path.join(build_dir(SRC_DIR),
            self.name, 'lib%s.a'%self.name)
        tmpobj = os.path.join(build_dir(SRC_DIR),
            self.name, 'lib%s.0.o'%self.name)

        render(makefile, '\n%s: %s\n'%(target, source))
        render(makefile, '\t@echo Creating \'%s\'\n'%prettify_target(target))
        render(makefile, '\t@%s %s %s 2> /dev/null\n'% \
            (objcopy, source, tmplib))
        render(makefile, '\t@cd %s && %s -x %s lib%s.0.o\n'% \
            (build_dir(self.path()), ar, tmplib, self.name))
        render(makefile, '\t@mv %s %s\n'%(tmpobj, target))
        render(makefile, '\t@rm %s\n'%tmplib)

    def render_sobj(self, makefile):
        if not self.asm_files: return
        source = ' '.join(self.asm_files)
        target = sobj_target(self.name)
        render(makefile, '\n%s: %s %s\n'% \
            (target, self.build_dirs_target(), source))
        render(makefile, '\t@echo Creating \'%s\'\n'%prettify_target(target))
        render(makefile, '\t@%s %s -o %s\n'%(gas, source, target))

    def render_mod(self, makefile):
        source = '%s %s'% (sobj_target(self.name), robj_target(self.name)) \
            if self.asm_files else robj_target(self.name)
        target = mod_target(self.name)
        lds = os.path.join(SRC_DIR, 'kernel', 'module.lds')
        render(makefile, '\n%s: %s\n'%(target, source))
        render(makefile, '\t@echo Creating \'%s\'\n'%prettify_target(target))
        render(makefile, '\t@%s -r -T %s %s -o %s\n'%(ld, lds, source, target))
        render(makefile, '\n.PHONY: %s.mod\n%s.mod: %s\n'% \
            (self.name, self.name, target))

data = read_build_json(os.path.join(SRC_DIR, 'build.json'))
kmodules = map(lambda n: Module(n), data['kernelModules'])
kernel_target = os.path.join(build_dir(SRC_DIR), 'kernel', 'arwen.ker')

def render_prolog(makefile):
    render(makefile, '# Generated by configure.py, do not modify\n')
    render(makefile, '\nROOT := $(shell dirname $(realpath' +
                   ' $(lastword $(MAKEFILE_LIST))))\n')
    render(makefile, '\n.PHONY: all\nall: arwen.ker\n')
    render(makefile, '\n.PHONY: clean\nclean:\n')
    render(makefile, '\t@echo Removing build dirs\n')
    render(makefile, '\t@rm -rf %s\n'%BUILD_DIR)

def render_kernel(makefile):
    source = ' '.join(map(lambda m: mod_target(m.name), kmodules))
    lds = os.path.join(SRC_DIR, 'kernel', 'arch-'+args.arch, 'kernel.lds')

    render(makefile, '\n# Kernel\n')
    render(makefile, '\n%s: %s\n'%(kernel_target, source))
    render(makefile, '\t@echo Creating \'%s\'\n'% \
        prettify_target(kernel_target))
    render(makefile, '\t@%s -nostdlib -z max-page-size=4096 -T %s %s -o %s\n' \
        %(ld, lds, source, kernel_target))
    render(makefile, '\n.PHONY: arwen.ker\narwen.ker: %s\n'%kernel_target)

def render_run(makefile):
    flags = ' -nographic'
    if args.arch == 'aarch64':
        flags += ' -machine type=virt -cpu cortex-a57'
    render(makefile, '\n# Run\n')
    render(makefile, '\n.PHONY: run\nrun: %s\n'%kernel_target)
    render(makefile, '\t@echo "Running QEMU (to exit press Ctrl-a x)"\n')
    render(makefile, '\t@%s%s -kernel %s\n'%(qemu, flags, kernel_target))

with open(os.path.join(ROOT_DIR, 'Makefile'), 'w') as f:
    render_prolog(f)

    for m in kmodules:
        f.write('\n')
        m.render(f)

    render_kernel(f)
    render_run(f)
