use core::fmt;

#[doc(hidden)]
pub enum Expected<'a, T> {
    Some,
    Value(&'a T),
    Message(&'static str),
}

impl<'a, T: fmt::Debug> fmt::Debug for Expected<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Some => f.write_str("`Some(..)`"),
            Self::Value(ref value) => write!(f, "`{:?}`", value),
            Self::Message(ref msg) => f.write_str(msg),
        }
    }
}

#[doc(hidden)]
pub enum Unexpected<'a, T> {
    Value(&'a T),
    Message(&'static str),
}

impl<'a, T: fmt::Debug> fmt::Debug for Unexpected<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(ref value) => write!(f, "`{:?}`", value),
            Self::Message(ref msg) => f.write_str(msg),
        }
    }
}

/// Internal function for `assert_*!` macros.
#[doc(hidden)]
pub fn assert_failed<'a, T>(
    expected: Expected<'a, T>,
    unexpected: Unexpected<'a, T>,
    args: Option<fmt::Arguments<'_>>,
) -> !
where
    T: fmt::Debug,
{
    match args {
        Some(args) => core::panic!(
            "assertion failed: expected {:?}, got {:?}\n{}",
            expected,
            unexpected,
            args,
        ),
        None => core::panic!(
            "assertion failed: expected {:?}, got {:?}",
            expected,
            unexpected,
        ),
    }
}
