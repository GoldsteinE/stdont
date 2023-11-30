use core::fmt;

/// Extensions for [`Result<T, E>`].
pub trait ResultExt<T, E> {
    /// Same as [`Result::unwrap()`], but uses [`fmt::Display`] instead of [`fmt::Debug`].
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

    /// Same as [`Result::expect()`], but uses [`fmt::Display`] instead of [`fmt::Debug`].
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
    /// Panics if the value is an [`Err`], with a panic message including the
    /// passed message, and the [`fmt::Display`]ed [`Err`].
    ///
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use std::io;
    /// # use stdont::ResultExt as _;
    /// #
    /// let x: Result<u32, _> = Err(io::Error::other("oh no!"));
    /// x.expect_display("Testing expect"); // panics with `Testing expect: oh no!`
    /// ```
    ///
    /// # Recommended Message Style
    ///
    /// We recommend that `expect` messages are used to describe the reason you
    /// _expect_ the `Result` should be `Ok`.
    ///
    /// ```should_panic
    /// # use stdont::ResultExt as _;
    /// #
    /// let path = std::env::var("IMPORTANT_PATH")
    ///     .expect_display("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
    /// ```
    ///
    /// **Hint**: If you're having trouble remembering how to phrase expect
    /// error messages remember to focus on the word "should" as in "env
    /// variable should be set by blah" or "the given binary should be available
    /// and executable by the current user".
    ///
    /// For more detail on expect message styles and the reasoning behind our recommendation please
    /// refer to the section on ["Common Message Styles"][1] in the [`std::error`] module docs.
    ///
    /// [1]: https://doc.rust-lang.org/stable/std/error/index.html#common-message-styles
    #[track_caller]
    fn expect_display(self, msg: &str) -> T
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

    #[inline]
    #[track_caller]
    fn expect_display(self, msg: &str) -> T
    where
        E: fmt::Display,
    {
        match self {
            Ok(t) => t,
            // PANIC: Intentional, required by the method semantics.
            #[allow(clippy::panic)]
            Err(e) => panic!("{msg}: {e}"),
        }
    }
}
