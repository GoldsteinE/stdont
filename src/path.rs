use std::path::{Path, PathBuf};

/// Extensions for [`PathBuf`].
///
/// See also [ACP #209](https://github.com/rust-lang/libs-team/issues/209)
pub trait PathBufExt {
    /// Join `path` to self and return self, so it can be chained.
    ///
    /// ```
    /// # use std::path::{Path, PathBuf};
    /// # use stdont::PathBufExt as _;
    /// let path = PathBuf::from("a").with("b").with("c");
    /// // Same thing, but with a lot more allocations
    /// assert_eq!(path, PathBuf::from("a").join("b").join("c"));
    /// ```
    #[must_use]
    fn with(self, other: impl AsRef<Path>) -> Self;
}

impl PathBufExt for PathBuf {
    #[inline]
    fn with(mut self, path: impl AsRef<Path>) -> Self {
        self.push(path);
        self
    }
}
