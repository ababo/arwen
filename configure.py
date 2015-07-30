#!/usr/bin/env python

import argparse
import json
import os.path
import platform
import re
import sys

ROOT_DIR = os.path.dirname(os.path.realpath(__file__))

HOST_OS = platform.system();
SUPPORTED_HOST_OSS = [ 'Darwin', 'Linux' ];

SUPPORTED_TARGET_ARCHS = [ 'x86_64', 'aarch64' ];
DEFAULT_TARGET_ARCH = platform.machine() \
    if platform.machine() in SUPPORTED_TARGET_ARCHS else 'x86_64'

SUPPORTED_OPT_LEVELS = [ 0, 1, 2, 3 ]
DEFAULT_OPT_LEVEL = 3

DEFAULT_BINUTILS_INFIXES = {
    'Darwin': '-elf-', 'Linux': '-unknown-linux-gnu-'
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
                    default=DEFAULT_BINUTILS_INFIX,
                    help='binutils prefix (default: <arch>%s)'% \
                        DEFAULT_BINUTILS_INFIX)
args = parser.parse_args()

if HOST_OS not in SUPPORTED_HOST_OSS:
    sys.stderr.write('error: unsupported host OS\n')
    sys.exit(1)

def read_build_json(filename):
    with open(filename, 'r') as f:
        return json.loads(f.read())

def find_extern_crates(filename):
    with open(filename, 'r') as f:
        return re.findall('extern\s+crate\s+(\w+)\s*;', f.read())

def build_dir(src_path):
    relpath = os.path.relpath(src_path, ROOT_DIR)
    return os.path.join(ROOT_DIR, 'build', relpath)

class Module:
    def __init__(self, name):
        self.name = name
        self.dependencies = find_extern_crates(
            os.path.join(self.path(), 'lib.rs'))
        self.asm_files = []
        self.rust_files =[]
        self.build_dirs = []
        self.find_all_files(self.path())

    def path(self):
        return os.path.join(ROOT_DIR, 'src', self.name)

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

    def render(self, makefile):
        makefile.write('# Module: %s\n'%self.name)
        self.render_build_dirs(makefile)

    def build_dirs_target(self):
        return os.path.join(build_dir(self.path()), '.build_dirs')

    def render_build_dirs(self, makefile):
        target = self.build_dirs_target()
        makefile.write('%s:\n'%target)
        makefile.write('\t@echo "Creating build dirs for \'%s\'"\n'%self.name)
        for d in self.build_dirs:
            makefile.write('\t@mkdir -p %s\n'%d)
        makefile.write('\t@touch %s\n'%target)

mod = Module('kernel')
with open(os.path.join(ROOT_DIR, 'Makefile'), 'w') as f:
    mod.render(f)
