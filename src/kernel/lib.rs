#![crate_name = "kernel"]
#![feature(asm, lang_items, linkage)]
#![no_std]

#[macro_use]
pub mod macros;

#[cfg(arch_x86_64)]
#[path = "arch-x86_64/mod.rs"]
pub mod arch;
#[cfg(arch_aarch64)]
#[path = "arch-aarch64/mod.rs"]
pub mod arch;

pub mod config;
pub mod klog;
pub mod libc;
pub mod memory;
pub mod util;

#[no_mangle]
#[lang = "begin_unwind"]
pub extern fn rust_begin_unwind(args: &core::fmt::Arguments,
                                file: &'static str, line: usize) -> ! {
	klog::log(klog::Level::Fatal,
		format_args!("panic: {} ({}:{})", args, file, line));
	extern { fn __halt() -> !; }
    unsafe { __halt(); }
}
