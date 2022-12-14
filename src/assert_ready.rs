/// Asserts that expression returns [`Ready`] variant.
///
/// [`Ready`]: core::task::Poll::Ready
#[macro_export]
macro_rules! assert_ready {
    ($cond:expr $(,)?) => {
        match $cond {
            core::task::Poll::Ready(t) => t,
            pending @ core::task::Poll::Pending => $crate::assert_failed!(
                $crate::panicking::Msg("`Ready(..)`"),
                $crate::panicking::Ref(&pending),
            ),
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            core::task::Poll::Ready(t) => t,
            pending @ core::task::Poll::Pending => $crate::assert_failed!(
                $crate::panicking::Msg("`Ready(..)`"),
                $crate::panicking::Ref(&pending),
                $($arg)+
            ),
        }
    };
}

/// Asserts that expression returns [`Ready`] variant.
///
/// [`Ready`]: core::task::Poll::Ready
#[macro_export]
macro_rules! debug_assert_ready {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_ready!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::task::Poll;

    #[test]
    #[should_panic(expected = "assertion failed: expected `Ready(..)`, got `Pending`")]
    fn check_assert_ready_panic() {
        assert_ready!(Poll::<bool>::Pending);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: expected `Ready(..)`, got `Pending`\ni'm confused"
    )]
    fn check_assert_ready_panic_with_message() {
        assert_ready!(Poll::<bool>::Pending, "i'm confused");
    }

    #[test]
    fn check_assert_ready_pass() {
        assert_ready!(Poll::Ready(true));
    }
}
