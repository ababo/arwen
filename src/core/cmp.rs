use marker::Sized;

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
