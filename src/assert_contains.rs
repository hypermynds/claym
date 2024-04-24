/// Asserts that the iterator contains a given element.
#[macro_export]
macro_rules! assert_contains {
    ($cond:expr, $expected:expr $(,)?) => {{
        if !$crate::panicking::check_contains($cond, &$expected) {
            core::panic!("assertion failed: expected to contain `{:?}`, but not found", $expected);
        }
    }};
    ($cond:expr, $expected:expr, $($arg:tt)+) => {{
        if !$crate::panicking::check_contains($cond, &$expected) {
            core::panic!("assertion failed: expected to contain `{:?}`, but not found\n{}", $expected, $($arg)+);
        }
    }};
}

/// Asserts that the iterator contains a given element.
#[macro_export]
macro_rules! debug_assert_contains {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_contains!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected to contain `4`, but not found")]
    fn check_assert_matches_panic() {
        let v = [1, 2, 3];
        assert_contains!(&v, 4);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: expected to contain `4`, but not found\ni'm confused"
    )]
    fn check_assert_matches_panic_with_message() {
        let v = [1, 2, 3];
        assert_contains!(&v, 4, "i'm confused");
    }

    #[test]
    fn check_assert_contains() {
        let v = [1, 2, 3];
        assert_contains!(&v, 3);
    }
}
