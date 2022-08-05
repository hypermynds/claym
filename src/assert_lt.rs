/// Asserts that first expression is less than the second.
#[macro_export]
macro_rules! assert_lt {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left, right) if !(*left < *right) => $crate::assert_comparison_failed!(
                $crate::panicking::Ref(left),
                $crate::panicking::Comparison::Less,
                $crate::panicking::Ref(right),
            ),
            _ => {}
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left, right) if !(*left < *right) => $crate::assert_comparison_failed!(
                $crate::panicking::Ref(left),
                $crate::panicking::Comparison::Less,
                $crate::panicking::Ref(right),
                $($arg)+
            ),
            _ => {}
        }
    };
}

/// Asserts that first expression is less than the second.
#[macro_export]
macro_rules! debug_assert_lt {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_lt!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: left < right\n left: `2`\nright: `1`")]
    fn check_assert_lt_panic() {
        assert_lt!(2, 1);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: left < right\n left: `2`\nright: `1`\ni'm confused"
    )]
    fn check_assert_lt_panic_with_message() {
        assert_lt!(2, 1, "i'm confused");
    }

    #[test]
    fn check_assert_lt_pass() {
        assert_lt!(1, 2);
    }
}
