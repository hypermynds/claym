/// Asserts that expression returns [`Err`] variant.
#[macro_export]
macro_rules! assert_err {
    ($cond:expr $(,)?) => {
        match $cond {
            Err(err) => err,
            Ok(ok) => $crate::assert_failed!(
                $crate::panicking::Msg("`Err(..)`"),
                $crate::panicking::Ref(&format_args!("Ok({:?})", ok)),
            ),
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Err(err) => err,
            Ok(ok) => $crate::assert_failed!(
                $crate::panicking::Msg("`Err(..)`"),
                $crate::panicking::Ref(&format_args!("Ok({:?})", ok)),
                $($arg)+
            ),
        }
    };
}

/// Asserts that expression returns [`Err`] variant.
#[macro_export]
macro_rules! debug_assert_err {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_err!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected `Err(..)`, got `Ok(true)`")]
    fn check_assert_err_panic() {
        assert_err!(Ok(true));
    }

    #[test]
    #[should_panic(expected = "assertion failed: expected `Err(..)`, got `Ok(true)`\ni'm confused")]
    fn check_assert_err_panic_with_message() {
        assert_err!(Ok(true), "i'm confused");
    }

    #[test]
    fn check_assert_err_pass() {
        assert_err!(Result::<bool, ()>::Err(()));
    }

    #[test]
    fn check_assert_err_does_not_require_err_debug() {
        struct Empty;

        assert_err!(Result::<(), Empty>::Err(Empty));
        assert_err!(Result::<(), Empty>::Err(Empty), "i'm confused");
    }
}
