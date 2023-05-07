# claym

[![Latest Version](https://img.shields.io/crates/v/claym.svg)](https://crates.io/crates/claym)
[![Latest Version](https://docs.rs/claym/badge.svg)](https://docs.rs/claym)
[![Build status](https://github.com/hypermynds/claym/actions/workflows/ci.yml/badge.svg)](https://github.com/hypermynds/claym/actions/workflows/ci.yml)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)
![no-std compatible](https://img.shields.io/badge/no--std-compatible-brightgreen)
![Version compatibility](https://img.shields.io/badge/Rust-1.56%2B-blue)

## Overview

This crate provides the same assertion macros as 
[claim](https://crates.io/crates/claim), but it has been developed without
dependencies (including build dependencies) and is intended for use only with
newer versions of Rust.

Add the following to your `Cargo.toml` manifest to replace `claim` with `claym`
for tests:

```toml
[dev-dependencies]
claim = { package = "claym", version = "0.4.0" }
```


## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or 
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
