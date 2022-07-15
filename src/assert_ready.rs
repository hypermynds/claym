/// Asserts that expression returns [`Poll::Ready`] variant.
///
/// [`Poll::Ready`]: https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! assert_ready {
    ($cond:expr $(,)?) => {
        match $cond {
            core::task::Poll::Ready(t) => t,
            pending @ core::task::Poll::Pending => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Ready(..)`"),
                    $crate::panicking::Value::Ref(&pending),
                    None,
                );
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            core::task::Poll::Ready(t) => t,
            pending @ core::task::Poll::Pending => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Ready(..)`"),
                    $crate::panicking::Value::Ref(&pending),
                    Some(format_args!($($arg)+)),
                );
            }
        }
    };
}

/// Asserts that expression returns [`Poll::Ready`] variant.
///
/// [`Poll::Ready`]: https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Ready
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
