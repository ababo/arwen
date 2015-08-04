use self::Option::*;

pub enum Option<T> {
    None,
    Some(T)
}

impl<T> Option<T> {
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
