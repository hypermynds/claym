use core::fmt;

#[doc(hidden)]
pub enum Value<'a, T> {
    Ref(&'a T),
    Str(&'static str),
}

impl<'a, T: fmt::Debug> fmt::Debug for Value<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ref(ref value) => write!(f, "`{:?}`", value),
            Self::Str(msg) => f.write_str(msg),
        }
    }
}

/// Internal function for `assert_*!` macros.
#[doc(hidden)]
pub fn assert_failed<T>(
    expected: Value<'_, T>,
    unexpected: Value<'_, T>,
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

/// Internal function for `assert_matches!` macro.
#[doc(hidden)]
pub fn assert_matches_failed<T>(
    unexpected: Value<'_, T>,
    variants: &'static str,
    args: Option<fmt::Arguments<'_>>,
) -> !
where
    T: fmt::Debug,
{
    match args {
        Some(args) => core::panic!(
            "assertion failed: expression does not match any of the given variants, got {:?}\nvariants: `{}`\n{}",
            unexpected,
            variants,
            args,
        ),
        None => core::panic!(
            "assertion failed: expression does not match any of the given variants, got {:?}\nvariants: `{}`",
            unexpected,
            variants,
        ),
    }
}

#[doc(hidden)]
pub enum Comparison {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Greater => f.write_str(">"),
            Self::GreaterEqual => f.write_str(">="),
            Self::Less => f.write_str("<"),
            Self::LessEqual => f.write_str("<="),
        }
    }
}

/// Internal function for `assert_*` comparison macros.
#[doc(hidden)]
pub fn assert_comparison_failed<L, R>(
    cmp: Comparison,
    left: Value<'_, L>,
    right: Value<'_, R>,
    args: Option<fmt::Arguments<'_>>,
) -> !
where
    L: fmt::Debug,
    R: fmt::Debug,
{
    match args {
        Some(args) => core::panic!(
            "assertion failed: left {} right\n left: {:?}\nright: {:?}\n{}",
            cmp,
            left,
            right,
            args,
        ),
        None => core::panic!(
            "assertion failed: left {} right\n left: {:?}\nright: {:?}",
            cmp,
            left,
            right,
        ),
    }
}
