/// Extensions for [`Option<T>`].
///
/// See also [ACP #212](https://github.com/rust-lang/libs-team/issues/212)
pub trait OptionExt<T> {
    /// Returns `true` if the option is a [`None`] or the value inside of it matches a predicate.
    ///
    /// ```rust
    /// # use stdont::OptionExt as _;
    ///
    /// let x: Option<u32> = Some(2);
    /// assert_eq!(x.is_none_or(|x| x > 1), true);
    ///
    /// let x: Option<u32> = Some(0);
    /// assert_eq!(x.is_none_or(|x| x > 1), false);
    ///
    /// let x: Option<u32> = None;
    /// assert_eq!(x.is_none_or(|x| x > 1), true);
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            None => true,
            Some(x) => f(x),
        }
    }
}
