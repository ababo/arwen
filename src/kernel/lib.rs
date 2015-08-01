#![crate_name = "kernel"]
#![feature(asm, no_std)]
#![no_std]

#[macro_use]
extern crate core;

#[cfg(arch_x86_64)]
#[path = "arch-x86_64/mod.rs"]
pub mod arch;
#[cfg(arch_aarch64)]
#[path = "arch-aarch64/mod.rs"]
pub mod arch;
