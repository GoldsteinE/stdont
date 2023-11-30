use core::fmt;

/// Extensions for [`Result<T, E>`].
pub trait ResultExt<T, E> {
    /// Same as [`Result::unwrap()], but uses [`fmt::Display`] instead of [`fmt::Debug`].
    ///
    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Err`]
    /// case explicitly, or call [`unwrap_or`], [`unwrap_or_else`], or
    /// [`unwrap_or_default`].
    ///
    /// [`unwrap_or`]: Result::unwrap_or
    /// [`unwrap_or_else`]: Result::unwrap_or_else
    /// [`unwrap_or_default`]: Result::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a [`fmt::Display`] panic message provided by the
    /// [`Err`]'s value.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use stdont::ResultExt as _;
    /// #
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.unwrap_display(), 2);
    /// ```
    ///
    /// ```should_panic
    /// # use std::io;
    /// # use stdont::ResultExt as _;
    /// #
    /// let x: Result<u32, _> = Err(io::Error::other("oh no!"));
    /// x.unwrap_display(); // panics with `oh no!`
    /// ```
    #[track_caller]
    fn unwrap_display(self) -> T
    where
        E: fmt::Display;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    #[inline]
    #[track_caller]
    fn unwrap_display(self) -> T
    where
        E: fmt::Display,
    {
        match self {
            Ok(t) => t,
            // PANIC: Intentional, required by the method semantics.
            #[allow(clippy::panic)]
            Err(e) => panic!("called `Result::unwrap()` on an error: {e}"),
        }
    }
}
