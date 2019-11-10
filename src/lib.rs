//! This crate provides an easy way to validate an IBAN (International Bank Account Number). To do
//! so, you can use the function [`parse()`](str::parse). This will check the IBAN rules
//! as well as the BBAN structure. The provided [`Iban`](crate::Iban) structure provides many methods
//! to easy the handling of an IBAN. Many of these methods are provided via the [`IbanLike`](crate::IbanLike)
//! trait.
//!
//! When parsing fails, the error type [`ParseIbanError`](crate::ParseIbanError) provides useful
//! information about what went wrong. If the BBAN could not be parsed, a [`BaseIban`](crate::BaseIban)
//! can still be used to access useful information.
//!
//! # Example
//! The following example does a full validation of the IBAN and BBAN format.
//!
//! ```rust
//! use iban::*;
//!
//! let account = "DE44500105175407324931".parse::<Iban>()?;
//!
//! assert_eq!(account.country_code(), "DE");
//! assert_eq!(account.check_digits(), 44);
//! assert_eq!(account.bban(), "500105175407324931");
//! assert_eq!(account.electronic_str(), "DE44500105175407324931");
//! assert_eq!(account.to_string(), "DE44 5001 0517 5407 3249 31");
//! # Ok::<(), iban::ParseIbanError>(())
//! ```
//!
//! # Features
//! - *serde*: Enable `serde` support for [`Iban`] and [`BaseIban`].

#![doc(html_root_url = "https://docs.rs/iban_validate/2.0.0")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::convert::TryFrom;
use std::fmt;
use std::str;
use thiserror::Error;

use crate::countries::RE_ADDRESS_REMAINDER;
use crate::countries::RE_COUNTRY_CODE;

mod base_iban;
mod countries;
#[cfg(test)]
mod tests;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use base_iban::{BaseIban, ParseBaseIbanError};

/// A trait that provide basic functions on an IBAN. It is implemented by both [`Iban`],
/// which represents a fully validated IBAN, and [`BaseIban`], which might not have a correct BBAN.
pub trait IbanLike {
    /// Get the IBAN in the electronic format, without whitespace. This method
    /// is simply a view into the inner string.
    ///
    /// # Example
    /// ```rust
    /// use iban::*;
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(iban.electronic_str(), "DE44500105175407324931");
    /// # Ok::<(), ParseIbanError>(())
    /// ```
    fn electronic_str(&self) -> &str;

    /// Get the country code of the IBAN. This method simply returns a slice of
    /// the inner representation.
    ///
    /// # Example
    /// ```rust
    /// use iban::*;
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(iban.country_code(), "DE");
    /// # Ok::<(), ParseIbanError>(())
    /// ```
    fn country_code(&self) -> &str {
        &self.electronic_str()[0..2]
    }

    /// Get the check digits of the IBAN, as a str. This method simply returns
    /// a slice of the inner representation. To obtain an integer instead,
    /// use [`check_digits`](IbanLike::check_digits).
    ///
    /// # Example
    /// ```rust
    /// use iban::*;
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(iban.check_digits_str(), "44");
    /// # Ok::<(), ParseIbanError>(())
    /// ```
    fn check_digits_str(&self) -> &str {
        &self.electronic_str()[2..4]
    }

    /// Get the check digits of the IBAN. This method parses the digits to an
    /// integer, performing slightly more work than [`check_digits_str`](IbanLike::check_digits_str).
    ///
    /// # Example
    /// ```rust
    /// use iban::*;
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(iban.check_digits(), 44);
    /// # Ok::<(), ParseIbanError>(())
    /// ```
    fn check_digits(&self) -> u8 {
        self.check_digits_str().parse().expect(
            "Could not parse check digits. Please create an issue at \
             https://github.com/ThomasdenH/iban_validate.",
        )
    }

    /// Get the BBAN part of the IBAN, as a `&str`. Note that the BBAN is not
    /// necessarily valid if this is not guaranteed by the implementing type.
    /// Use [`Iban::bban`] to guarantee a correct BBAN.
    ///
    /// # Example
    /// ```rust
    /// use iban::*;
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(iban.bban_unchecked(), "500105175407324931");
    /// # Ok::<(), ParseIbanError>(())
    /// ```
    fn bban_unchecked(&self) -> &str {
        &self.electronic_str()[4..]
    }
}

impl IbanLike for Iban {
    fn electronic_str(&self) -> &str {
        self.base_iban.electronic_str()
    }
}

impl Iban {
    /// Get the BBAN part of the IBAN, as a `&str`. This method, in contrast to [`IbanLike::bban_unchecked`],
    /// is only available on the [`Iban`] structure, which means the returned BBAN string is always correct.
    ///
    /// # Example
    /// ```rust
    /// use iban::*;
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(iban.bban(), "500105175407324931");
    /// # Ok::<(), ParseIbanError>(())
    /// ```
    pub fn bban(&self) -> &str {
        self.bban_unchecked()
    }

    /// Get the bank identifier of the IBAN. The bank identifier might not be
    /// defined, in which case this method returns `None`.
    ///
    /// # Example
    /// ```
    /// use iban::*;
    /// let iban: Iban = "AD12 0001 2030 2003 5910 0100".parse()?;
    /// assert_eq!(iban.bank_identifier(), Some("0001"));
    /// # Ok::<(), ParseIbanError>(())
    /// ```
    pub fn bank_identifier(&self) -> Option<&str> {
        match self.country_code() {
            "AD" => Some(0..4),
            "AE" => Some(0..3),
            "AL" => Some(0..3),
            _ => panic!("unknown country"),
        }
        .map(|range| &self.electronic_str()[4..][range])
    }

    /// Get the branch identifier of the IBAN. The branch identifier might not be
    /// defined, in which case this method returns `None`.
    ///
    /// # Example
    /// ```
    /// use iban::*;
    /// let iban: Iban = "AD12 0001 2030 2003 5910 0100".parse()?;
    /// assert_eq!(iban.branch_identifier(), Some("2030"));
    /// # Ok::<(), ParseIbanError>(())
    /// ```
    pub fn branch_identifier(&self) -> Option<&str> {
        match self.country_code() {
            "AD" => Some(4..8),
            "AE" => None,
            "AL" => Some(3..7),
            _ => panic!("unknown country"),
        }
        .map(|range| &self.electronic_str()[4..][range])
    }
}

impl From<Iban> for BaseIban {
    fn from(value: Iban) -> BaseIban {
        value.base_iban
    }
}

impl fmt::Debug for Iban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.base_iban, f)
    }
}

impl fmt::Display for Iban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.base_iban, f)
    }
}

/// Represents an IBAN. To obtain it, make use of the [`parse()`] function, which will make sure the
/// string follows the ISO 13616 standard. Apart from its own methods, `Iban` implements [`IbanLike`],
/// which provides more functionality.
///
/// The impementation of [`Display`](std::fmt::Display) provides spaced formatting of the IBAN. Electronic
/// formatting can be obtained via [`electronic_str`](IbanLike::electronic_str).
///
/// A valid IBAN satisfies the defined format, has a valid checksum and has a BBAN format as defined in the
/// IBAN registry.
///
/// # Examples
/// ```rust
/// use iban::*;
/// let address = "KZ86125KZT5004100100".parse::<iban::Iban>()?;
/// assert_eq!(address.to_string(), "KZ86 125K ZT50 0410 0100");
/// # Ok::<(), iban::ParseIbanError>(())
/// ```
///
/// [`parse()`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Iban {
    /// The inner IBAN, which has been checked.
    base_iban: BaseIban,
}

/// An error indicating the IBAN could not be parsed.
///
/// # Example
/// ```rust
/// use iban::{BaseIban, Iban, ParseIbanError, ParseBaseIbanError};
/// use std::convert::TryFrom;
///
/// // The following IBAN has an invalid checksum
/// assert_eq!(
///     "MR00 0002 0001 0100 0012 3456 754".parse::<Iban>(),
///     Err(ParseIbanError::from(ParseBaseIbanError::InvalidChecksum))
/// );
///
/// // The following IBAN doesn't follow the country format
/// let base_iban: BaseIban = "AL84212110090000AB023569874".parse()?;
/// assert_eq!(
///     Iban::try_from(base_iban),
///     Err(ParseIbanError::InvalidBban(base_iban))
/// );
/// # Ok::<(), ParseBaseIbanError>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Error)]
pub enum ParseIbanError {
    /// This variant indicates that the basic IBAN structure was not followed.
    #[error("the string does not follow the base IBAN rules")]
    InvalidBaseIban {
        /// The error indicating what went wrong when parsing the Iban.
        #[from]
        source: ParseBaseIbanError,
    },
    /// This variant indicates that the BBAN did not follow the correct format.
    /// The `BaseIban` provides functionality on the IBAN part of the
    /// address.
    #[error("the IBAN doesn't have a correct BBAN")]
    InvalidBban(BaseIban),
    /// This variant indicated that the country code of the IBAN was not recognized.
    /// The `BaseIban` provides functionality on the IBAN part of the
    /// address.
    #[error("the IBAN country code wasn't recognized")]
    UnknownCountry(BaseIban),
}

impl<'a> TryFrom<&'a str> for Iban {
    type Error = ParseIbanError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value
            .parse::<BaseIban>()
            .map_err(ParseIbanError::from)
            .and_then(Iban::try_from)
    }
}

impl TryFrom<BaseIban> for Iban {
    type Error = ParseIbanError;
    fn try_from(base_iban: BaseIban) -> Result<Iban, ParseIbanError> {
        let country_match = RE_COUNTRY_CODE
            .matches(base_iban.country_code())
            .iter()
            .next();

        if let Some(country_index) = country_match {
            let address_match = RE_ADDRESS_REMAINDER
                .matches(base_iban.bban_unchecked())
                .iter()
                .find(|&address_index| address_index == country_index);

            if address_match.is_some() {
                Ok(Iban { base_iban })
            } else {
                Err(ParseIbanError::InvalidBban(base_iban))
            }
        } else {
            Err(ParseIbanError::UnknownCountry(base_iban))
        }
    }
}

impl str::FromStr for Iban {
    type Err = ParseIbanError;
    fn from_str(address: &str) -> Result<Self, Self::Err> {
        Iban::try_from(address)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Iban {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.base_iban.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Iban {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IbanStringVisitor;
        use serde::de;

        impl<'vi> de::Visitor<'vi> for IbanStringVisitor {
            type Value = Iban;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "an IBAN string")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Iban, E> {
                value.parse::<Iban>().map_err(E::custom)
            }
        }

        deserializer.deserialize_str(IbanStringVisitor)
    }
}
