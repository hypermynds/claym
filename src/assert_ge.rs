/// Asserts that first expression is greater or equal than the second.
#[macro_export]
macro_rules! assert_ge {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left, right) if !(*left >= *right) => $crate::assert_comparison_failed!(
                $crate::panicking::Ref(left),
                $crate::panicking::Comparison::GreaterEqual,
                $crate::panicking::Ref(right),
            ),
            _ => {}
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left, right) if !(*left >= *right) => $crate::assert_comparison_failed!(
                $crate::panicking::Ref(left),
                $crate::panicking::Comparison::GreaterEqual,
                $crate::panicking::Ref(right),
                $($arg)+
            ),
            _ => {}
        }
    };
}

/// Asserts that first expression is greater or equal than the second.
#[macro_export]
macro_rules! debug_assert_ge {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_ge!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: left >= right\n left: `1`\nright: `2`")]
    fn check_assert_ge_panic() {
        assert_ge!(1, 2);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: left >= right\n left: `1`\nright: `2`\ni'm confused"
    )]
    fn check_assert_ge_panic_with_message() {
        assert_ge!(1, 2, "i'm confused");
    }

    #[test]
    fn check_assert_ge_pass() {
        assert_ge!(2, 1);
        assert_ge!(2, 2);
    }
}
