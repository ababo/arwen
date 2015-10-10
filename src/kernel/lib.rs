#![crate_name = "kernel"]
#![feature(asm, no_std, lang_items, core_str_ext)]
#![no_std]

#[macro_use]
pub mod klog;

#[cfg(arch_x86_64)]
#[path = "arch-x86_64/mod.rs"]
pub mod arch;
#[cfg(arch_aarch64)]
#[path = "arch-aarch64/mod.rs"]
pub mod arch;

pub mod libc;

#[no_mangle]
#[lang = "begin_unwind"]
pub extern fn rust_begin_unwind(_: &core::fmt::Arguments,
                                _: &'static str, _: usize) -> ! {
    loop {}
}
