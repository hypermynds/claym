/// Asserts that first expression is less or equal than the second.
#[macro_export]
macro_rules! assert_le {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left, right) if !(*left <= *right) => $crate::assert_comparison_failed!(
                $crate::panicking::Ref(left),
                $crate::panicking::Comparison::LessEqual,
                $crate::panicking::Ref(right),
            ),
            _ => {}
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left, right) if !(*left <= *right) => $crate::assert_comparison_failed!(
                $crate::panicking::Ref(left),
                $crate::panicking::Comparison::LessEqual,
                $crate::panicking::Ref(right),
                $($arg)+
            ),
            _ => {}
        }
    };
}

/// Asserts that first expression is less or equal than the second.
#[macro_export]
macro_rules! debug_assert_le {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_le!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: left <= right\n left: `2`\nright: `1`")]
    fn check_assert_le_panic() {
        assert_le!(2, 1);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: left <= right\n left: `2`\nright: `1`\ni'm confused"
    )]
    fn check_assert_le_panic_with_message() {
        assert_le!(2, 1, "i'm confused");
    }

    #[test]
    fn check_assert_le_pass() {
        assert_le!(1, 2);
        assert_le!(2, 2);
    }
}
