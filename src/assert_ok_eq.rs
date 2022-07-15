/// Asserts that expression returns [`Ok`] variant and its value equals to the right expression.
#[macro_export]
macro_rules! assert_ok_eq {
    ($cond:expr, $expected:expr $(,)?) => {{
        let value = $crate::assert_ok!($cond);
        assert_eq!(value, $expected);
        value
    }};
    ($cond:expr, $expected:expr, $($arg:tt)+) => {{
        let value = $crate::assert_ok!($cond, $($arg)+);
        assert_eq!(value, $expected);
        value
    }};
}

/// Asserts that expression returns [`Ok`] variant and its value equals to the right expression.
#[macro_export]
macro_rules! debug_assert_ok_eq {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_ok_eq!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected `Ok(..)`, got `Err(())`")]
    fn check_assert_ok_eq_panic() {
        assert_ok_eq!(Result::<bool, _>::Err(()), true);
    }

    #[test]
    #[should_panic]
    fn check_assert_ok_eq_panic_not_equal() {
        assert_ok_eq!(Result::<_, ()>::Ok(false), true);
    }

    #[test]
    #[should_panic(expected = "assertion failed: expected `Ok(..)`, got `Err(())`\ni'm confused")]
    fn check_assert_some_eq_panic_with_message() {
        assert_ok_eq!(Result::<bool, _>::Err(()), true, "i'm confused");
    }

    #[test]
    fn check_assert_ok_eq_pass() {
        assert_ok_eq!(Result::<_, ()>::Ok(true), true);
    }
}
