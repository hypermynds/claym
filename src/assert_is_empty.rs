/// Asserts that the iterator is empty.
#[macro_export]
macro_rules! assert_is_empty {
    ($cond:expr $(,)?) => {{
        if !$crate::panicking::check_is_empty($cond) {
            core::panic!("assertion failed: expected to be empty");
        }
    }};
    ($cond:expr, $($arg:tt)+) => {{
        if !$crate::panicking::check_is_empty($cond) {
            core::panic!("assertion failed: expected to be empty\n{}", $($arg)+);
        }
    }};
}

/// Asserts that the iterator is empty.
#[macro_export]
macro_rules! debug_assert_is_empty {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_is_empty!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected to be empty")]
    fn check_assert_is_empty_panic() {
        let v = [1, 2, 3];
        assert_is_empty!(&v);
    }

    #[test]
    #[should_panic(expected = "assertion failed: expected to be empty\ni'm confused")]
    fn check_assert_is_empty_panic_with_message() {
        let v = [1, 2, 3];
        assert_is_empty!(&v, "i'm confused");
    }

    #[test]
    fn check_assert_is_empty() {
        let v: [u8; 0] = [];
        assert_is_empty!(&v);
    }
}
