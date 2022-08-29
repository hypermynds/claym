/// Asserts that expression returns [`Ok`] variant.
#[macro_export]
macro_rules! assert_ok {
    ($cond:expr $(,)?) => {
        match $cond {
            Ok(value) => value,
            Err(err) => $crate::assert_failed!(
                $crate::panicking::Msg("`Ok(..)`"),
                $crate::panicking::Ref(&format_args!("Err({:?})", err)),
            ),
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(value) => value,
            Err(err) => $crate::assert_failed!(
                $crate::panicking::Msg("`Ok(..)`"),
                $crate::panicking::Ref(&format_args!("Err({:?})", err)),
                $($arg)+
            ),
        }
    };
}

/// Asserts that expression returns [`Ok`] variant.
#[macro_export]
macro_rules! debug_assert_ok {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_ok!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected `Ok(..)`, got `Err(())`")]
    fn check_assert_ok_panic() {
        assert_ok!(Result::<bool, ()>::Err(()));
    }

    #[test]
    #[should_panic(expected = "assertion failed: expected `Ok(..)`, got `Err(())`\ni'm confused")]
    fn check_assert_ok_panic_with_message() {
        assert_ok!(Result::<bool, ()>::Err(()), "i'm confused");
    }

    #[test]
    fn check_assert_ok_pass() {
        assert_ok!(Result::<_, ()>::Ok(true));
    }

    #[test]
    fn check_assert_ok_does_not_require_ok_debug() {
        struct Empty;

        assert_ok!(Result::<_, ()>::Ok(Empty));
        assert_ok!(Result::<_, ()>::Ok(Empty), "i'm confused");
    }
}
