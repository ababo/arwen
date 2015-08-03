extern "rust-intrinsic" {
    pub fn discriminant_value<T>(v: &T) -> u64;
    pub fn unreachable() -> !;
}
