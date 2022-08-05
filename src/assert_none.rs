/// Asserts that expression returns [`None`] variant.
#[macro_export]
macro_rules! assert_none {
    ($cond:expr $(,)?) => {
        match $cond {
            none @ None => none,
            some @ Some(_) => $crate::assert_failed!(
                $crate::panicking::Msg("`None`"),
                $crate::panicking::Ref(&some),
            ),
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            none @ None => none,
            some @ Some(_) => $crate::assert_failed!(
                $crate::panicking::Msg("`None`"),
                $crate::panicking::Ref(&some),
                $($arg)+
            ),
        }
    };
}

/// Asserts that expression returns [`None`] variant.
#[macro_export]
macro_rules! debug_assert_none {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_none!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: expected `None`, got `Some(true)`")]
    fn check_assert_none_panic() {
        assert_none!(Some(true));
    }

    #[test]
    #[should_panic(expected = "assertion failed: expected `None`, got `Some(true)`\ni'm confused")]
    fn check_assert_none_panic_with_message() {
        assert_none!(Some(true), "i'm confused");
    }

    #[test]
    fn check_assert_none_pass() {
        assert_none!(Option::<bool>::None);
    }
}
