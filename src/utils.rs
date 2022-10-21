#[must_use]
#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}
