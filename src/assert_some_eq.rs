/// Asserts that expression returns [`Some`] variant and its value equals to the right expression.
#[macro_export]
macro_rules! assert_some_eq {
    ($cond:expr, $expected:expr $(,)?) => {{
        let value = $crate::assert_some!($cond);
        assert_eq!(value, $expected);
        value
    }};
    ($cond:expr, $expected:expr, $($arg:tt)+) => {{
        let value = $crate::assert_some!($cond, $($arg)+);
        assert_eq!(value, $expected);
        value
    }};
}

/// Asserts that expression returns [`Some`] variant and its value equals to the right expression.
#[macro_export]
macro_rules! debug_assert_some_eq {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_some_eq!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected `Some(..)`, got `None`")]
    fn check_assert_some_eq_panic() {
        assert_some_eq!(Option::<bool>::None, true);
    }

    #[test]
    #[should_panic]
    fn check_assert_some_eq_panic_not_equal() {
        assert_some_eq!(Some(false), true);
    }

    #[test]
    #[should_panic(expected = "assertion failed: expected `Some(..)`, got `None`\ni'm confused")]
    fn check_assert_some_eq_panic_with_message() {
        assert_some_eq!(Option::<bool>::None, true, "i'm confused");
    }

    #[test]
    fn check_assert_some_eq_pass() {
        assert_some_eq!(Some(true), true);
    }
}
