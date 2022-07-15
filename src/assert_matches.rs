/// Asserts that expression matches any of the given variants.
#[macro_export]
macro_rules! assert_matches {
    ($expression:expr, $($pattern:pat_param)|+ $(if $guard:expr)? $(,)?) => {
        match $expression {
            $($pattern)|+ $(if $guard)? => {},
            other => $crate::panicking::assert_matches_failed(
                $crate::panicking::Value::Ref(&other),
                stringify!($($pattern)|+ $(if $guard)?),
                None,
            )
        }
    };
    ($expression:expr, $($pattern:pat_param)|+ $(if $guard:expr)?, $($arg:tt)+) => {
        match $expression {
            $($pattern)|+ $(if $guard)? => {},
            other => $crate::panicking::assert_matches_failed(
                $crate::panicking::Value::Ref(&other),
                stringify!($($pattern)|+ $(if $guard)?),
                Some(format_args!($($arg)+)),
            )
        }
    };
}

/// Asserts that expression matches any of the given variants.
#[macro_export]
macro_rules! debug_assert_matches {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_matches!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(
        expected = "assertion failed: expression does not match any of the given variants, got `Err(())`\nvariants: `Ok(true)`"
    )]
    fn check_assert_matches_panic() {
        assert_matches!(Result::<bool, ()>::Err(()), Ok(true));
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: expression does not match any of the given variants, got `Err(())`\nvariants: `Ok(true)`\ni'm confused"
    )]
    fn check_assert_matches_panic_with_message() {
        assert_matches!(Result::<bool, ()>::Err(()), Ok(true), "i'm confused");
    }

    #[test]
    fn check_assert_matches_pass() {
        assert_matches!(Result::<_, ()>::Ok(true), Ok(true));
    }
}
