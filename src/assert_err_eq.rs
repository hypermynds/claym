/// Asserts that expression returns [`Err`] variant and its value equals to the right expression.
#[macro_export]
macro_rules! assert_err_eq {
    ($cond:expr, $expected:expr $(,)?) => {{
        let value = $crate::assert_err!($cond);
        assert_eq!(value, $expected);
        value
    }};
    ($cond:expr, $expected:expr, $($arg:tt)+) => {{
        let value = $crate::assert_err!($cond, $($arg)+);
        assert_eq!(value, $expected);
        value
    }};
}

/// Asserts that expression returns [`Err`] variant and its value equals to the right expression.
#[macro_export]
macro_rules! debug_assert_err_eq {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_err_eq!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected `Err(..)`, got `Ok(())`")]
    fn check_assert_err_eq_panic() {
        assert_err_eq!(Result::<_, bool>::Ok(()), true);
    }

    #[test]
    #[should_panic]
    fn check_assert_err_eq_panic_not_equal() {
        assert_err_eq!(Result::<(), _>::Err(false), true);
    }

    #[test]
    #[should_panic(expected = "assertion failed: expected `Err(..)`, got `Ok(())`\ni'm confused")]
    fn check_assert_err_eq_panic_with_message() {
        assert_err_eq!(Result::<_, bool>::Ok(()), true, "i'm confused");
    }

    #[test]
    fn check_assert_err_eq_pass() {
        assert_err_eq!(Result::<(), _>::Err(true), true);
    }
}
