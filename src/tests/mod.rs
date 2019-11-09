//! This module contains unit tests

type TestResult = Result<(), Box<dyn std::error::Error>>;

mod format;
mod impls;
mod parse;
mod proptest;
#[cfg(feature = "serde")]
mod serde;
mod split;
mod validate_country;
