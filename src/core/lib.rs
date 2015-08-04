#![crate_name = "core"]
#![feature(intrinsics, lang_items, no_std)]
#![no_std]

#[macro_export]
macro_rules! panic {
    () => (
        panic!("explicit panic")
    );
    ($msg:expr) => ({
        loop {}
    });
    ($fmt:expr, $($arg:tt)*) => ({
        loop {}
    });
}

pub mod cmp;
pub mod intrinsics;
pub mod marker;
pub mod ops;
pub mod option;
pub mod prelude;
pub mod result;
