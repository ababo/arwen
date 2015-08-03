use marker::Sized;
use option::Option::{self, None, Some};
use self::Ordering::*;

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    #[inline]
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

impl PartialEq for () {
    #[inline]
    fn eq(&self, _other: &()) -> bool { true }
    #[inline]
    fn ne(&self, _other: &()) -> bool { false }
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

partial_eq_impl! {
    bool char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64
}

pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}

#[lang = "ord"]
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    #[inline]
    fn lt(&self, other: &Rhs) -> bool {
        match self.partial_cmp(other) {
            Some(Less) => true,
            _ => false,
        }
    }

    #[inline]
    fn le(&self, other: &Rhs) -> bool {
        match self.partial_cmp(other) {
            Some(Less) | Some(Equal) => true,
            _ => false,
        }
    }

    #[inline]
    fn gt(&self, other: &Rhs) -> bool {
        match self.partial_cmp(other) {
            Some(Greater) => true,
            _ => false,
        }
    }

    #[inline]
    fn ge(&self, other: &Rhs) -> bool {
        match self.partial_cmp(other) {
            Some(Greater) | Some(Equal) => true,
            _ => false,
        }
    }
}

macro_rules! partial_ord_impl {
    ($($t:ty)*) => ($(
        impl PartialOrd for $t {
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                match (self <= other, self >= other) {
                    (false, false) => None,
                    (false, true) => Some(Greater),
                    (true, false) => Some(Less),
                    (true, true) => Some(Equal),
                }
            }
            #[inline]
            fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
            #[inline]
            fn le(&self, other: &$t) -> bool { (*self) <= (*other) }
            #[inline]
            fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
            #[inline]
            fn gt(&self, other: &$t) -> bool { (*self) > (*other) }
        }
    )*)
}

impl PartialOrd for () {
    #[inline]
    fn partial_cmp(&self, _: &()) -> Option<Ordering> {
        Some(Equal)
    }
}

impl PartialOrd for bool {
    #[inline]
    fn partial_cmp(&self, other: &bool) -> Option<Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}

partial_ord_impl! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

impl<'a, 'b, A: ?Sized, B: ?Sized> PartialEq<&'b B> for &'a A
    where A: PartialEq<B> {
    #[inline]
    fn eq(&self, other: & &'b B) -> bool { PartialEq::eq(*self, *other) }
    #[inline]
    fn ne(&self, other: & &'b B) -> bool { PartialEq::ne(*self, *other) }
}

impl<'a, 'b, A: ?Sized, B: ?Sized> PartialOrd<&'b B> for &'a A
    where A: PartialOrd<B> {
    #[inline]
    fn partial_cmp(&self, other: &&'b B) -> Option<Ordering> {
        PartialOrd::partial_cmp(*self, *other)
    }
    #[inline]
    fn lt(&self, other: & &'b B) -> bool { PartialOrd::lt(*self, *other) }
    #[inline]
    fn le(&self, other: & &'b B) -> bool { PartialOrd::le(*self, *other) }
    #[inline]
    fn ge(&self, other: & &'b B) -> bool { PartialOrd::ge(*self, *other) }
    #[inline]
    fn gt(&self, other: & &'b B) -> bool { PartialOrd::gt(*self, *other) }
}

impl<'a, 'b, A: ?Sized, B: ?Sized> PartialEq<&'b mut B> for &'a mut A
    where A: PartialEq<B> {
    #[inline]
    fn eq(&self, other: &&'b mut B) -> bool { PartialEq::eq(*self, *other) }
    #[inline]
    fn ne(&self, other: &&'b mut B) -> bool { PartialEq::ne(*self, *other) }
}

impl<'a, 'b, A: ?Sized, B: ?Sized> PartialOrd<&'b mut B> for &'a mut A
    where A: PartialOrd<B> {
    #[inline]
    fn partial_cmp(&self, other: &&'b mut B) -> Option<Ordering> {
        PartialOrd::partial_cmp(*self, *other)
    }
    #[inline]
    fn lt(&self, other: &&'b mut B) -> bool { PartialOrd::lt(*self, *other) }
    #[inline]
    fn le(&self, other: &&'b mut B) -> bool { PartialOrd::le(*self, *other) }
    #[inline]
    fn ge(&self, other: &&'b mut B) -> bool { PartialOrd::ge(*self, *other) }
    #[inline]
    fn gt(&self, other: &&'b mut B) -> bool { PartialOrd::gt(*self, *other) }
}
