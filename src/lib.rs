#![no_std]

//! Assertion macros.
//!
//! This crate provides additional assert macros to make testing easier. `claym` does not have
//! depndencies and it can be used in a `no-std` environment.
//!
//! All the macros in this crate has `debug_*` counterparts, like any assertion macros in the
//! standard libraries.
//!
//! Assertions similar to [`assert_eq!`] or [`assert_ne!`]:
//!
//! * [`assert_ge!`]
//! * [`assert_gt!`]
//! * [`assert_le!`]
//! * [`assert_lt!`]
//! * [`assert_matches!`]
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
//!
//! Assertions for [`Poll`] variants:
//!
//! * [`assert_pending!`]
//! * [`assert_ready!`]
//! * [`assert_ready_eq!`]
//! * [`assert_ready_err!`]
//! * [`assert_ready_ok!`]
//!
//! [`Poll`]: core::task::Poll

mod assert_err;
mod assert_ge;
mod assert_gt;
mod assert_le;
mod assert_lt;
mod assert_matches;
mod assert_none;
mod assert_ok;
mod assert_ok_eq;
mod assert_pending;
mod assert_ready;
mod assert_ready_eq;
mod assert_ready_err;
mod assert_ready_ok;
mod assert_some;
mod assert_some_eq;
#[doc(hidden)]
pub mod panicking;
