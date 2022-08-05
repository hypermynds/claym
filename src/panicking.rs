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
#[doc(hidden)]
pub struct Ref<'a, T>(pub &'a T);

impl<'a, T> fmt::Debug for Ref<'a, T>
where
    T: fmt::Debug,
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{:?}`", self.0)
    }
}

#[doc(hidden)]
pub struct Msg(pub &'static str);

impl fmt::Debug for Msg {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
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

/// Internal macro for `assert_*!` macros.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_failed {
    ($expected:expr, $unexpected:expr, $($arg:tt)+) => {
        core::panic!(
            "assertion failed: expected {:?}, got {:?}\n{}",
            $expected,
            $unexpected,
            core::format_args!($($arg)+)
        )
    };
    ($expected:expr, $unexpected:expr $(,)?) => {
        core::panic!(
            "assertion failed: expected {:?}, got {:?}",
            $expected,
            $unexpected
        )
    };
}

/// Internal function for `assert_matches!` macro.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_matches_failed {
    ($unexpected:expr, $variants:expr, $($arg:tt)+) => {
        core::panic!(
            "assertion failed: expression does not match any of the given variants, got {:?}\nvariants: `{}`\n{}",
            $unexpected,
            $variants,
            core::format_args!($($arg)+)
        )
    };
    ($unexpected:expr, $variants:expr $(,)?) => {
        core::panic!(
            "assertion failed: expression does not match any of the given variants, got {:?}\nvariants: `{}`",
            $unexpected,
            $variants,
        )
    };
}

/// Internal function for `assert_*` comparison macros.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_comparison_failed {
    ($left:expr, $cmp:expr, $right:expr, $($arg:tt)+) => {
        core::panic!(
            "assertion failed: left {} right\n left: {:?}\nright: {:?}\n{}",
            $cmp,
            $left,
            $right,
            core::format_args!($($arg)+)
        )
    };
    ($left:expr, $cmp:expr, $right:expr $(,)?) => {
        core::panic!(
            "assertion failed: left {} right\n left: {:?}\nright: {:?}",
            $cmp,
            $left,
            $right,
        )
    };
}
