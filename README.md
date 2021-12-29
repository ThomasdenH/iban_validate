# iban_validate

[![Crates.io](https://img.shields.io/crates/v/iban_validate.svg)](https://crates.io/crates/iban_validate)
[![Travis Build Status](https://travis-ci.org/ThomasdenH/iban_validate.svg?branch=master)](https://travis-ci.org/ThomasdenH/iban_validate)
[![Appveyor Build Status](https://ci.appveyor.com/api/projects/status/github/ThomasdenH/iban_validate?svg=true)](https://ci.appveyor.com/project/ThomasdenH/iban-validate)
[![Coverage Status](https://coveralls.io/repos/github/ThomasdenH/iban_validate/badge.svg?branch=master)](https://coveralls.io/github/ThomasdenH/iban_validate?branch=master)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/ThomasdenH/iban_validate.svg)](http://isitmaintained.com/project/ThomasdenH/iban_validate "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/ThomasdenH/iban_validate.svg)](http://isitmaintained.com/project/ThomasdenH/iban_validate "Percentage of issues still open")

This crate provides an easy way to validate an IBAN (International Bank Account Number). To do so, you can use the function [`parse()`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse). This will check the IBAN rules as well as the BBAN structure. The provided [`Iban`] structure provides many methods to easy the handling of an IBAN. Many of these methods are provided via the [`IbanLike`](https://docs.rs/iban_validate/4.0.0/iban/trait.IbanLike.html) trait.

When BBAN parsing fails, the error type [`ParseIbanError`](https://docs.rs/iban_validate/4.0.0/iban/enum.ParseIbanError.html) provides useful information about what went wrong. Additionally, the error contains [`BaseIban`], which can still be used to access useful information.

## Example

The following example does a full validation of the IBAN and BBAN format.

```rust
use iban::*;

let account = "DE44500105175407324931".parse::<Iban>()?;

assert_eq!(account.country_code(), "DE");
assert_eq!(account.check_digits(), 44);
assert_eq!(account.bban(), "500105175407324931");
assert_eq!(account.electronic_str(), "DE44500105175407324931");
assert_eq!(account.to_string(), "DE44 5001 0517 5407 3249 31");
assert_eq!(account.bank_identifier(), Some("50010517"));
assert_eq!(account.branch_identifier(), None);
```

## What does this library provide?

- A [`Iban`] type that can be used to parse account numbers very quickly. It doesn't require allocations at all, and instead leverages [`arrayvec`](https://crates.io/crates/arrayvec) under the hood.
- A flexible API that is useful even when the country is not in the Swift registry (using [`BaseIban`]. Instead of using panic, the crate provides typed errors with what went wrong.
- All functionality can be used in a `no_std` environment (except for the implementation of `std` traits).
- Optional serialization and deserialization via [`serde`](https://crates.io/crates/serde).
- CI tested results via the Swift provided and custom test cases, as well as proptest.
- `#![forbid(unsafe_code)]`, making sure all code is written in safe Rust.

## Usage

The crate can be found on [crates.io](https://crates.io/crates/iban_validate). To use this crate, just add it as an
dependency:

```toml
[dependencies]
iban_validate = "4"
```

## Features

The following features can be used to configure the crate:

- _std_: **Enabled by default.** Enable the standard library. It is only used to provide implementations of [`Error`](https://doc.rust-lang.org/stable/std/error/trait.Error.html).
- _serde_: Enable `serde` support for [`Iban`] and [`BaseIban`].

## Documentation

The full documentation is available at [docs.rs](https://docs.rs/iban_validate/).

## Contributing

If you experience issues with this crate or want to help, please look [here](contributing.md).

## Stability

This crate has the goal of being usable on the latest stable version of Rust. Its minimum version is documented and tested against, although no extra effort is taken to support earlier versions. The minimum version of Rust can be increased, although this corresponds to a new major release. The crate does not use unsafe itself, although dependencies might. Breaking changes are not avoided in new major versions, although these will always be well-documented and the process of upgrading made clear.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`iban`]: https://docs.rs/iban_validate/4.0.0/iban/struct.Iban.html
[`baseiban`]: https://docs.rs/iban_validate/4.0.0/iban/struct.BaseIban.html
