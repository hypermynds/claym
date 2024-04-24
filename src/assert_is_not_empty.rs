/// Asserts that the iterator is not empty.
#[macro_export]
macro_rules! assert_is_not_empty {
    ($cond:expr $(,)?) => {{
        if $crate::panicking::check_is_empty($cond) {
            core::panic!("assertion failed: expected to be non-empty");
        }
    }};
    ($cond:expr, $($arg:tt)+) => {{
        if $crate::panicking::check_is_empty($cond) {
            core::panic!("assertion failed: expected to be non-empty\n{}", $($arg)+);
        }
    }};
}

/// Asserts that the iterator is not empty.
#[macro_export]
macro_rules! debug_assert_is_not_empty {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_is_not_empty!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected to be non-empty")]
    fn check_assert_is_empty_panic() {
        let v: [u8; 0] = [];
        assert_is_not_empty!(&v);
    }

    #[test]
    #[should_panic(expected = "assertion failed: expected to be non-empty\ni'm confused")]
    fn check_assert_is_empty_panic_with_message() {
        let v: [u8; 0] = [];
        assert_is_not_empty!(&v, "i'm confused");
    }

    #[test]
    fn check_assert_is_empty() {
        let v = [1, 2, 3];
        assert_is_not_empty!(&v);
    }
}
