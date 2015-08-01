#![crate_name = "kernel"]
#![feature(asm, no_std, lang_items)]
#![no_std]

#[macro_use]
extern crate core;

#[cfg(arch_x86_64)]
#[path = "arch-x86_64/mod.rs"]
pub mod x86_64;
#[cfg(arch_aarch64)]
#[path = "arch-aarch64/mod.rs"]
pub mod aarch64;

#[lang = "sized"]
pub trait Sized {}

#[lang="copy"]
pub trait Copy {}


#[lang = "add"]
pub trait Add<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for i32 {
    type Output = i32;
    #[inline]
    fn add(self, rhs: i32) -> i32 { self + rhs }
}

impl Add for u32 {
    type Output = u32;
    #[inline]
    fn add(self, rhs: u32) -> u32 { self + rhs }
}

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    /// This method tests for `self` and `other` values to be equal, and is used
    /// by `==`.
    fn eq(&self, other: &Rhs) -> bool;

    /// This method tests for `!=`.
    #[inline]
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

macro_rules! partial_eq_impl {
        ($($t:ty)*) => ($(
            impl PartialEq for $t {
                #[inline]
                fn eq(&self, other: &$t) -> bool { (*self) == (*other) }
                #[inline]
                fn ne(&self, other: &$t) -> bool { (*self) != (*other) }
            }
        )*)
    }

    impl PartialEq for () {
        #[inline]
        fn eq(&self, _other: &()) -> bool { true }
        #[inline]
        fn ne(&self, _other: &()) -> bool { false }
    }

    partial_eq_impl! {
        bool char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64
    }