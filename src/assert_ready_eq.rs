/// Asserts that expression returns [`Ready`] variant and its value equals to the right expression.
///
/// [`Ready`]: core::task::Poll::Ready
#[macro_export]
macro_rules! assert_ready_eq {
    ($cond:expr, $expected:expr $(,)?) => {{
        let value = $crate::assert_ready!($cond);
        assert_eq!(value, $expected);
        value
    }};
    ($cond:expr, $expected:expr, $($arg:tt)+) => {{
        let value = $crate::assert_ready!($cond, $($arg)+);
        assert_eq!(value, $expected);
        value
    }};
}

/// Asserts that expression returns [`Ready`] variant and its value equals to the right expression.
///
/// [`Ready`]: core::task::Poll::Ready
#[macro_export]
macro_rules! debug_assert_ready_eq {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_ready_eq!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::task::Poll;

    #[test]
    #[should_panic(expected = "assertion failed: expected `Ready(..)`, got `Pending`")]
    fn check_assert_ready_eq_panic() {
        assert_ready_eq!(Poll::<bool>::Pending, true);
    }

    #[test]
    #[should_panic]
    fn check_assert_ready_eq_panic_not_equal() {
        assert_ready_eq!(Poll::Ready(false), true);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: expected `Ready(..)`, got `Pending`\ni'm confused"
    )]
    fn check_assert_ready_eq_panic_with_message() {
        assert_ready_eq!(Poll::<bool>::Pending, true, "i'm confused");
    }

    #[test]
    fn check_assert_ready_eq_pass() {
        assert_ready_eq!(Poll::Ready(true), true);
    }
}
