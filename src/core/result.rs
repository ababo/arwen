use self::Result::*;

pub enum Result<T, E> {
    Ok(T),
    Err(E)
}

impl<T, E> Result<T, E> {
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(_e) =>
                panic!("called `Result::unwrap()` on an `Err` value: {:?}", e)
        }
    }
}
