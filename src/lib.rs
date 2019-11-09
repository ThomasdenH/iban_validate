//! This crate provides an easy way to validate an IBAN (International Bank Account Number). To do
//! so, you can use the function [`parse()`]. If you want to check whether the address has a valid
//! BBAN (Basic Bank Account Number), you can use [`validate_bban()`]. It also contains some
//! helper methods to make handling an IBAN easier.
//!
//! # Example
//! The following example does a full validation of the IBAN and BBAN format.
//!
//! ```rust
//! use iban::Iban;
//! use iban::BbanResult;
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
//! [`parse()`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
//! [`validate_bban()`]: struct.Iban.html#method.validate_bban

// Crate doesn't use unsafe itself, but the lazy_static macro uses #![allow(unsafe_code)], so use
// deny instead of forbid
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/iban_validate/2.0.0")]
#![forbid(unsafe_code)]

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::ops;
use std::str;

use crate::countries::RE_ADDRESS_REMAINDER;
use crate::countries::RE_COUNTRY_CODE;

mod base_iban;
mod countries;
#[cfg(test)]
mod tests;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use base_iban::{BaseIban, ParseBaseIbanError};

/// A trait that provide basic functions on an IBAN.
pub trait IbanLike {
    /// Get the IBAN in the electronic format, without whitespace.
    ///
    /// # Example
    /// ```rust
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(account.electronic_str(), "DE44500105175407324931");
    /// # Ok::<(), ParseIbanError>
    /// ```
    fn electronic_str(&self) -> &str;

    /// Get the country code of the IBAN.
    ///
    /// # Example
    /// ```rust
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(account.country_code(), "DE");
    /// # Ok::<(), ParseIbanError>
    /// ```
    fn country_code(&self) -> &str {
        &self.electronic_str()[0..2]
    }

    /// Get the check digits of the IBAN, as a str.
    ///
    /// # Example
    /// ```rust
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(account.check_digits_str(), "44");
    /// # Ok::<(), ParseIbanError>
    /// ```
    fn check_digits_str(&self) -> &str {
        &self.electronic_str()[2..4]
    }

    /// Get the check digits of the IBAN.
    ///
    /// # Example
    /// ```rust
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(account.check_digits(), 44);
    /// # Ok::<(), ParseIbanError>
    /// ```
    fn check_digits(&self) -> u8 {
        self.check_digits_str().parse().expect(
            "Could not parse check digits. Please create an issue at \
             https://github.com/ThomasdenH/iban_validate.",
        )
    }

    /// Get the BBAN part of the IBAN, as a `&str`. Note that the BBAN is not
    /// necessarily valid if this is not guaranteed by the implementing type.
    /// Use `Iban::bban` to guarantee a correct BBAN.
    ///
    /// # Example
    /// ```rust
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(account.bban_unchecked(), "500105175407324931");
    /// # Ok::<(), ParseIbanError>
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
    /// Get the BBAN part of the IBAN, as a `&str`.
    ///
    /// # Example
    /// ```rust
    /// let iban: Iban = "DE44 5001 0517 5407 3249 31".parse()?;
    /// assert_eq!(account.bban(), "500105175407324931");
    /// # Ok::<(), ParseIbanError>
    /// ```
    pub fn bban(&self) -> &str {
        self.bban_unchecked()
    }

    /// Obtain the inner `BaseIban`.
    pub fn into_base_iban(self) -> BaseIban {
        self.base_iban
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

impl ops::Deref for dyn IbanLike {
    type Target = str;

    fn deref(&self) -> &str {
        self.electronic_str()
    }
}

/// Represents an IBAN. To obtain it, make use of the [`parse()`] function, which will make sure the
/// string follows the ISO 13616 standard.
///
/// # Examples
/// ```rust
/// let address = "KZ86125KZT5004100100".parse::<iban::Iban>()?;
/// # Ok::<(), iban::ParseIbanError>(())
/// ```
///
/// [`parse()`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Iban {
    /// The inner IBAN, which has been checked.
    base_iban: BaseIban,
}

/// An error indicating the Iban could not be parsed.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ParseIbanError {
    /// This variant indicates that the basic IBAN structure was not followed.
    InvalidBaseIban {
        /// The error indicating what went wrong when parsing the Iban.
        source: ParseBaseIbanError,
    },
    /// This variant indicates that the BBAN did not follow the correct format.
    /// The `BaseIban` provides functionality on the IBAN part of the
    /// address.
    InvalidBban(BaseIban),
    /// This variant indicated that the country code of the IBAN was not recognized.
    /// The `BaseIban` provides functionality on the IBAN part of the
    /// address.
    UnknownCountry(BaseIban),
}

impl fmt::Display for ParseIbanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseIbanError::*;
        match self {
            InvalidBaseIban { .. } => write!(f, "the string is not a valid IBAN"),
            InvalidBban(_) => write!(f, "the string has an invalid BBAN"),
            UnknownCountry(_) => write!(f, "the country code of the IBAN was not recognized"),
        }
    }
}
impl Error for ParseIbanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let ParseIbanError::InvalidBaseIban { source } = self {
            Some(source)
        } else {
            None
        }
    }
}

impl str::FromStr for Iban {
    type Err = ParseIbanError;
    fn from_str(address: &str) -> Result<Self, Self::Err> {
        let base_iban: BaseIban = address
            .parse()
            .map_err(|source| ParseIbanError::InvalidBaseIban { source })?;

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

impl<'a> TryFrom<&'a str> for Iban {
    type Error = ParseIbanError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.parse()
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
