/// Asserts that expression returns [`Ok`] variant.
#[macro_export]
macro_rules! assert_ok {
    ($cond:expr $(,)?) => {
        match $cond {
            Ok(value) => value,
            err @ Err(..) => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Ok(..)`"),
                    $crate::panicking::Value::Ref(&err),
                    None,
                );
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(value) => value,
            err @ Err(..) => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Ok(..)`"),
                    $crate::panicking::Value::Ref(&err),
                    Some(format_args!($($arg)+)),
                );
            }
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
}
