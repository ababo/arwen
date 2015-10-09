### Building instructions

Arwen OS can be built in the following host environments: Linux and Mac OS for x86\_64. It can run on the following target CPU architectures: x86_64 and aarch64. Make sure you have a recent version of a host OS - this will greatly simplify the build preparations.

#####1. Install Rust Nightly

Typically you can just run:
	
```bash
curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --	channel=nightly
``` 

#####2. Install binutils for a chosen target

It can be done easily in recent Linux versions just using a package manager. For older ones you'll probably need to manually install binutils package for aarch64 or even to build the tools yourself. The same applies to Mac OS: binutils for aarch64 should be built from the recent [sources](http://ftp.gnu.org/gnu/binutils/).

The typical command to build and install binutils for aarch64:

```bash
./configure --target aarch64-elf && make && sudo make install
```

#####3. Install QEMU for a chosen target (if you need an emulation)

This is fairly simple for x86_64 target (just use your packet manager, e.g. apt, yum, macports, ...), but is still more complicated for aarch64 (you need a QEMU version 2.2 or higher). In the second case you'll probably need to build it from the recent [sources](http://wiki.qemu.org/Download).

The typical command to build and install QEMU for aarch64:

```bash
./configure --target-list=aarch64-softmmu --enable-system --disable-user && make && sudo make install
```
