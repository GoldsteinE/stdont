/// Extensions for [`bool`].
///
/// See also [ACP #188](https://github.com/rust-lang/libs-team/issues/188)
pub trait BoolExt {
    /// Logical implication, equivalent to `!self || other`.
    ///
    /// ```rust
    /// # use stdont::BoolExt as _;
    /// assert!(false.implies(false));
    /// assert!(false.implies(true));
    /// assert!(!true.implies(false));
    /// assert!(true.implies(true));
    /// ```
    #[must_use]
    fn implies(self, other: Self) -> Self;

    /// Inverse of [`.implies()`][BoolExt::implies].
    ///
    /// ```rust
    /// # use stdont::BoolExt as _;
    /// assert!(false.implied_by(false));
    /// assert!(!false.implied_by(true));
    /// assert!(true.implied_by(false));
    /// assert!(true.implied_by(true));
    /// ```
    #[must_use]
    fn implied_by(self, other: Self) -> Self;
}

impl BoolExt for bool {
    #[inline]
    fn implies(self, other: Self) -> Self {
        !self || other
    }

    #[inline]
    fn implied_by(self, other: Self) -> Self {
        !other || self
    }
}
