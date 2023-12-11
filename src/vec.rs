/// Extensions for [`Vec<T>`].
pub trait VecExt<T> {
    /// Fallible version of [`.remove()`][Vec::remove].
    fn try_remove(&mut self, idx: usize) -> Option<T>;
    /// Fallible version of [`.swap_remove()`][Vec::swap_remove].
    fn try_swap_remove(&mut self, idx: usize) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {
    #[inline]
    fn try_remove(&mut self, idx: usize) -> Option<T> {
        (idx < self.len()).then(|| self.remove(idx))
    }

    #[inline]
    fn try_swap_remove(&mut self, idx: usize) -> Option<T> {
        (idx < self.len()).then(|| self.swap_remove(idx))
    }
}
