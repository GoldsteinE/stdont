/// Like [`assert_eq!()`], but the left argument has semantics of "expected value".
///
/// Panic message looks like this:
///
/// ```text
/// assertion failed: `(expected == actual)`
/// expected: 2
///   actual: 3
/// ```
#[macro_export]
macro_rules! expect_eq {
    ($expected:expr, $actual:expr $(,)?) => {
        assert!(
            $expected == $actual,
            "assertion failed: `(expected == actual)`\nexpected: {:?}\n  actual: {:?}",
            $expected,
            $actual
        )
    };
}

#[cfg(test)]
#[allow(clippy::unwrap_used)] // it's test code, duh
mod tests {
    use crate::expect_eq;

    #[test]
    fn expect_eq() {
        let panic = std::panic::catch_unwind(|| expect_eq!(2, 3)).unwrap_err();
        let message = *panic.downcast::<String>().unwrap();
        assert_eq!(
            message,
            "assertion failed: `(expected == actual)`\nexpected: 2\n  actual: 3",
        );
    }
}
