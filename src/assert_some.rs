/// Asserts that expression returns [`Some`] variant.
#[macro_export]
macro_rules! assert_some {
    ($cond:expr $(,)?) => {
        match $cond {
            Some(value) => value,
            none @ None => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Some(..)`"),
                    $crate::panicking::Value::Ref(&none),
                    None,
                );
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Some(value) => value,
            none @ None => {
                $crate::panicking::assert_failed(
                    $crate::panicking::Value::Str("`Some(..)`"),
                    $crate::panicking::Value::Ref(&none),
                    Some(format_args!($($arg)+)),
                );
            }
        }
    };
}

/// Asserts that expression returns [`Some`] variant.
#[macro_export]
macro_rules! debug_assert_some {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_some!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected `Some(..)`, got `None`")]
    fn check_assert_some_panic() {
        assert_some!(Option::<bool>::None);
    }

    #[test]
    #[should_panic(expected = "assertion failed: expected `Some(..)`, got `None`\ni'm confused")]
    fn check_assert_some_panic_with_message() {
        assert_some!(Option::<bool>::None, "i'm confused");
    }

    #[test]
    fn check_assert_some_pass() {
        assert_some!(Some(true));
    }
}
