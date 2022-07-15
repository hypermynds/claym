#![no_std]

//! Assertion macros.
//!
//! This crate provides additional assert macros to make testing easier. `claym` does not have
//! depndencies and it can be used in a `no-std` environment.
//!
//! All the macros in this crate has `debug_*` counterparts, like any assertion macros in the
//! standard libraries.
//!
//! Assertions for [`Result`] variants:
//!
//! * [`assert_err!`]
//! * [`assert_ok!`]
//! * [`assert_ok_eq!`]
//!
//! Assertions for [`Option`] variants:
//!
//! * [`assert_none!`]
//! * [`assert_some!`]
//! * [`assert_some_eq!`]

mod assert_err;
mod assert_none;
mod assert_ok;
mod assert_ok_eq;
mod assert_some;
mod assert_some_eq;
#[doc(hidden)]
pub mod panicking;
