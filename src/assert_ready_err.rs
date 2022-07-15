/// Asserts that expression returns [`Ready(Err(..))`] variant.
///
/// [`Ready(Err(..))`]: core::task::Poll::Ready
#[macro_export]
macro_rules! assert_ready_err {
    ($cond:expr $(,)?) => {
        match $cond {
            core::task::Poll::Ready(Err(err)) => err,
            poll => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Ready(Err(..))`"),
                    $crate::panicking::Value::Ref(&poll),
                    None,
                );
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            core::task::Poll::Ready(Err(err)) => err,
            poll => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Ready(Err(..))`"),
                    $crate::panicking::Value::Ref(&poll),
                    Some(format_args!($($arg)+)),
                );
            }
        }
    };
}

/// Asserts that expression returns [`Ready(Err(..))`] variant.
///
/// [`Ready(Err(..))`]: core::task::Poll::Ready
#[macro_export]
macro_rules! debug_assert_ready_err {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_ready_err!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::task::Poll;

    #[test]
    #[should_panic(expected = "assertion failed: expected `Ready(Err(..))`, got `Ready(Ok(true))`")]
    fn check_assert_ready_err_panic() {
        assert_ready_err!(Poll::Ready(Ok(true)));
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: expected `Ready(Err(..))`, got `Ready(Ok(true))`\ni'm confused"
    )]
    fn check_assert_ready_panic_with_message() {
        assert_ready_err!(Poll::Ready(Ok(true)), "i'm confused");
    }

    #[test]
    fn check_assert_ready_err_pass() {
        assert_ready_err!(Poll::Ready(Result::<bool, _>::Err(())));
    }
}
