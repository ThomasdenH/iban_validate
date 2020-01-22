//! This module statically checks whether types implement the expected traits.
use iban::{BaseIban, Iban, ParseBaseIbanError, ParseIbanError};
use static_assertions::assert_impl_all;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

assert_impl_all!(
    BaseIban: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Display,
    FromStr,
    TryFrom<&'static str>,
    Send,
    Sync
);
assert_impl_all!(
    Iban: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Display,
    FromStr,
    TryFrom<BaseIban>,
    TryFrom<&'static str>,
    Send,
    Sync
);
assert_impl_all!(
    ParseBaseIbanError: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Display,
    Error,
    Send,
    Sync
);
assert_impl_all!(
    ParseIbanError: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Display,
    Error,
    Send,
    Sync
);

#[cfg(feature = "serde")]
mod impls_serde {
    use super::*;
    use serde::{Deserialize, Serialize};
    assert_impl_all!(BaseIban: Serialize, Deserialize<'static>);
    assert_impl_all!(Iban: Serialize, Deserialize<'static>);
}
