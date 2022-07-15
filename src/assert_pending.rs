/// Asserts that expression returns [`Poll::Pending`] variant.
///
/// [`Poll::Pending`]: core::task::Poll::Pending
#[macro_export]
macro_rules! assert_pending {
    ($cond:expr $(,)?) => {
        match $cond {
            pending @ core::task::Poll::Pending => pending,
            ready @ core::task::Poll::Ready(_) => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Ref(&core::task::Poll::Pending),
                    $crate::panicking::Value::Ref(&ready),
                    None,
                );
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            pending @ core::task::Poll::Pending => pending,
            ready @ core::task::Poll::Ready(_) => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Ref(&core::task::Poll::Pending),
                    $crate::panicking::Value::Ref(&ready),
                    Some(format_args!($($arg)+)),
                );
            }
        }
    };
}

/// Asserts that expression returns [`Poll::Pending`] variant.
///
/// [`Poll::Pending`]: core::task::Poll::Pending
#[macro_export]
macro_rules! debug_assert_pending {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_peding!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::task::Poll;

    #[test]
    #[should_panic(expected = "assertion failed: expected `Pending`, got `Ready(true)`")]
    fn check_assert_pending_panic() {
        let _ = assert_pending!(Poll::Ready(true));
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: expected `Pending`, got `Ready(true)`\ni'm confused"
    )]
    fn check_assert_pending_panic_with_message() {
        let _ = assert_pending!(Poll::Ready(true), "i'm confused");
    }

    #[test]
    fn check_assert_pending_pass() {
        let _ = assert_pending!(Poll::<bool>::Pending);
    }
}
