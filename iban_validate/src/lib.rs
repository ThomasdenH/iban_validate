#![doc = include_str!("../README.md")]
#![doc(html_root_url = "https://docs.rs/iban_validate/5.0.1")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![deny(missing_debug_implementations)]
#![no_std]

use core::convert::TryFrom;
use core::error::Error;
use core::fmt::{Display, Debug, self};
use core::str;

mod base_iban;
mod countries;
mod generated;
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

    /// Get the check digits of the IBAN, as a string slice. This method simply returns
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
        generated::bank_identifier(self.country_code())
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
        generated::branch_identifier(self.country_code())
            .map(|range| &self.electronic_str()[4..][range])
    }
}

impl From<Iban> for BaseIban {
    fn from(value: Iban) -> BaseIban {
        value.base_iban
    }
}

impl Debug for Iban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.base_iban, f)
    }
}

impl Display for Iban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.base_iban, f)
    }
}

/// Represents an IBAN. To obtain it, make use of the [`parse()`] function, which will make sure the
/// string follows the ISO 13616 standard. Apart from its own methods, `Iban` implements [`IbanLike`],
/// which provides more functionality.
///
/// The impementation of [`Display`] provides spaced formatting of the IBAN. Electronic
/// formatting can be obtained via [`electronic_str`](IbanLike::electronic_str).
///
/// A valid IBAN...
/// - must start with two uppercase ASCII letters, followed
/// by two digits, followed by any number of digits and ASCII
/// letters.
/// - must have a valid checksum.
/// - must contain no whitespace, or be in the paper format, where
///   characters are in space-separated groups of four.
/// - must adhere to the country-specific format.
///
/// Sometimes it may be desirable to accept IBANs that do not have their
/// country registered in the IBAN registry, or it may simply be unimportant
/// whether the country's BBAN format was followed. In that case, you can use
/// a [`BaseIban`] instead.
/// 
/// # Examples
/// ```rust
/// use iban::*;
/// let address = "KZ86125KZT5004100100".parse::<iban::Iban>()?;
/// assert_eq!(address.to_string(), "KZ86 125K ZT50 0410 0100");
/// # Ok::<(), iban::ParseIbanError>(())
/// ```
///
/// ## Formatting
/// The IBAN specification describes two formats: an electronic format without
/// whitespace and a paper format which seperates the IBAN in groups of
/// four characters. Both will be parsed correctly by this crate. When
/// formatting, [`Debug`] can be used to output the former and [`Display`] for
/// the latter. This is true for a [`BaseIban`] as well as an [`Iban`].
/// Alternatively, you can use [`IbanLike::electronic_str`] to obtain the
/// electronic format as a string slice.
/// ```
/// # use iban::ParseIbanError;
/// let iban: iban::Iban = "RO66BACX0000001234567890".parse()?;
/// // Use Debug for the electronic format.
/// assert_eq!(&format!("{:?}", iban), "RO66BACX0000001234567890");
/// // Use Display for the pretty print format.
/// assert_eq!(&format!("{}", iban), "RO66 BACX 0000 0012 3456 7890");
/// # Ok::<(), ParseIbanError>(())
/// ```
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
/// use core::convert::TryFrom;
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
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl From<ParseBaseIbanError> for ParseIbanError {
    fn from(source: ParseBaseIbanError) -> ParseIbanError {
        ParseIbanError::InvalidBaseIban { source }
    }
}

impl fmt::Display for ParseIbanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParseIbanError::InvalidBaseIban { .. } =>
                    "the string does not follow the base IBAN rules",
                ParseIbanError::InvalidBban(..) => "the IBAN doesn't have a correct BBAN",
                ParseIbanError::UnknownCountry(..) => "the IBAN country code wasn't recognized",
            }
        )
    }
}

impl Error for ParseIbanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseIbanError::InvalidBaseIban { source } => Some(source),
            _ => None,
        }
    }
}

impl<'a> TryFrom<&'a str> for Iban {
    type Error = ParseIbanError;
    /// Parse an IBAN without taking the BBAN into consideration.
    ///
    /// # Errors
    /// If the string does not match the IBAN format or the checksum is
    /// invalid, [`ParseIbanError::InvalidBaseIban`] will be
    /// returned. If the country format is invalid or unknown, the other
    /// variants will be returned with the [`BaseIban`] giving
    /// access to some basic functionality nonetheless.
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value
            .parse::<BaseIban>()
            .map_err(|source| ParseIbanError::InvalidBaseIban { source })
            .and_then(Iban::try_from)
    }
}

impl TryFrom<BaseIban> for Iban {
    type Error = ParseIbanError;
    /// Parse an IBAN without taking the BBAN into consideration.
    ///
    /// # Errors
    /// If the string does not match the IBAN format or the checksum is
    /// invalid, [`ParseIbanError::InvalidBaseIban`] will be
    /// returned. If the country format is invalid or unknown, the other
    /// variants will be returned with the [`BaseIban`] giving
    /// access to some basic functionality nonetheless.
    fn try_from(base_iban: BaseIban) -> Result<Iban, ParseIbanError> {
        use countries::Matchable;
        generated::country_pattern(base_iban.country_code())
            .ok_or(ParseIbanError::UnknownCountry(base_iban))
            .and_then(|matcher: &[(usize, _)]| {
                if matcher.match_str(base_iban.bban_unchecked()) {
                    Ok(Iban { base_iban })
                } else {
                    Err(ParseIbanError::InvalidBban(base_iban))
                }
            })
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
