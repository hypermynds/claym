use core::{borrow::Borrow, fmt};

#[doc(hidden)]
pub struct Ref<'a, T>(pub &'a T);

impl<'a, T> fmt::Display for Ref<'a, T>
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

impl fmt::Display for Msg {
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
            "assertion failed: expected {}, got {}\n{}",
            $expected,
            $unexpected,
            core::format_args!($($arg)+)
        )
    };
    ($expected:expr, $unexpected:expr $(,)?) => {
        core::panic!(
            "assertion failed: expected {}, got {}",
            $expected,
            $unexpected
        )
    };
}

/// Internal function for `assert_*` comparison macros.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_comparison_failed {
    ($left:expr, $cmp:expr, $right:expr, $($arg:tt)+) => {
        core::panic!(
            "assertion failed: left {} right\n left: {}\nright: {}\n{}",
            $cmp,
            $left,
            $right,
            core::format_args!($($arg)+)
        )
    };
    ($left:expr, $cmp:expr, $right:expr $(,)?) => {
        core::panic!(
            "assertion failed: left {} right\n left: {}\nright: {}",
            $cmp,
            $left,
            $right,
        )
    };
}

#[doc(hidden)]
pub fn check_contains<I, B>(container: I, expected: B) -> bool
where
    I: IntoIterator,
    B: Borrow<I::Item>,
    I::Item: PartialEq + fmt::Debug,
{
    container
        .into_iter()
        .any(|element| element.eq(expected.borrow()))
}

#[doc(hidden)]
pub fn check_is_empty<I>(container: I) -> bool
where
    I: IntoIterator,
{
    container.into_iter().next().is_none()
}

#[doc(hidden)]
pub fn check_has_length<I>(container: I, length: usize) -> Option<usize>
where
    I: IntoIterator,
{
    let container_length = container.into_iter().count();

    if container_length == length {
        None
    } else {
        Some(container_length)
    }
}
