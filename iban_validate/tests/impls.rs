//! This module statically checks whether types implement the expected traits.
use core::convert::TryFrom;
use core::fmt::{Debug, Display};
use core::hash::Hash;
use core::str::FromStr;
use iban::{BaseIban, Iban, ParseBaseIbanError, ParseIbanError};
use static_assertions::assert_impl_all;

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
    Sync,
    From<Iban>,
    AsRef<BaseIban>,
    AsMut<BaseIban>
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
    Sync,
    Into<BaseIban>,
    // We can convert between references. We cannot convert between mutable
    // references, since a changed BaseIban may not be a valid Iban anymore.
    AsRef<BaseIban>,
    AsRef<Iban>,
    AsMut<Iban>
);
assert_impl_all!(
    ParseBaseIbanError: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Send,
    Sync,
    Display,
    Into<ParseIbanError>,
    AsRef<ParseBaseIbanError>,
    AsMut<ParseBaseIbanError>
);
assert_impl_all!(
    ParseIbanError: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Send,
    Sync,
    Display,
    From<ParseBaseIbanError>,
    AsRef<ParseIbanError>,
    AsMut<ParseIbanError>
);

assert_impl_all!(ParseBaseIbanError: core::error::Error);
assert_impl_all!(ParseIbanError: core::error::Error);

#[cfg(feature = "serde")]
mod impls_serde {
    use super::{assert_impl_all, BaseIban, Iban};
    use serde::{Deserialize, Serialize};
    assert_impl_all!(BaseIban: Serialize, Deserialize<'static>);
    assert_impl_all!(Iban: Serialize, Deserialize<'static>);
}
