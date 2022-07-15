/// Asserts that expression returns [`Ready(Ok(..))`] variant.
///
/// [`Ready(Ok(..))`]: core::task::Poll::Ready
#[macro_export]
macro_rules! assert_ready_ok {
    ($cond:expr $(,)?) => {
        match $cond {
            core::task::Poll::Ready(Ok(ok)) => ok,
            poll => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Ready(Ok(..))`"),
                    $crate::panicking::Value::Ref(&poll),
                    None,
                );
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            core::task::Poll::Ready(Ok(ok)) => ok,
            poll => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Ready(Ok(..))`"),
                    $crate::panicking::Value::Ref(&poll),
                    Some(format_args!($($arg)+)),
                );
            }
        }
    };
}

/// Asserts that expression returns [`Ready(Ok(..))`] variant.
///
/// [`Ready(Ok(..))`]: core::task::Poll::Ready
#[macro_export]
macro_rules! debug_assert_ready_ok {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_ready_ok!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::task::Poll;

    #[test]
    #[should_panic(expected = "assertion failed: expected `Ready(Ok(..))`, got `Ready(Err(()))`")]
    fn check_assert_ready_ok_panic() {
        assert_ready_ok!(Poll::Ready(Result::<bool, _>::Err(())));
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: expected `Ready(Ok(..))`, got `Ready(Err(()))`\ni'm confused"
    )]
    fn check_assert_ready_panic_with_message() {
        assert_ready_ok!(Poll::Ready(Result::<bool, _>::Err(())), "i'm confused");
    }

    #[test]
    fn check_assert_ready_ok_pass() {
        assert_ready_ok!(Poll::Ready(Result::<_, ()>::Ok(true)));
    }
}
