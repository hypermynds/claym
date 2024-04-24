/// Asserts that the iterator has the given length.
#[macro_export]
macro_rules! assert_has_length {
    ($cond:expr, $expected:expr $(,)?) => {{
        let expected_len = $expected;
        if let Some(unexpected_len) = $crate::panicking::check_has_length($cond, expected_len) {
            core::panic!(
                "assertion failed: expected a sequence of length {}, got {}",
                expected_len,
                unexpected_len,
            );
        }
    }};
    ($cond:expr, $expected:expr, $($arg:tt)+) => {{
        let expected_len = $expected;
        if let Some(unexpected_len) = $crate::panicking::check_has_length($cond, expected_len) {
            core::panic!(
                "assertion failed: expected a sequence of length {}, got {}\n{}",
                expected_len,
                unexpected_len,
                $($arg)+,
            );
        }
    }};
}

/// Asserts that the iterator has the given length.
#[macro_export]
macro_rules! debug_assert_has_length {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_has_length!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected a sequence of length 4, got 3")]
    fn check_assert_is_empty_panic() {
        let v = [1, 2, 3];
        assert_has_length!(&v, 4);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: expected a sequence of length 4, got 3\ni'm confused"
    )]
    fn check_assert_is_empty_panic_with_message() {
        let v = [1, 2, 3];
        assert_has_length!(&v, 4, "i'm confused");
    }

    #[test]
    fn check_assert_is_empty() {
        let v = [1, 2, 3];
        assert_has_length!(&v, 3);
    }
}
