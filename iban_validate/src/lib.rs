#![doc = include_str!("../README.md")]
#![doc(html_root_url = "https://docs.rs/iban_validate/4.0.1")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![deny(missing_debug_implementations)]
#![no_std]

use core::convert::TryFrom;
use core::error::Error;
use core::fmt;
use core::str;

mod base_iban;
mod countries;
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
        #[allow(clippy::match_same_arms)] // For clarity, identical arms are not combined.
        match self.country_code() {
            "AD" => Some(0..4),
            "AE" => Some(0..3),
            // The bank identifier length (8) does not match the range (0..3) in the registry. Using length as truth.
            "AL" => Some(0..8),
            "AT" => Some(0..5),
            "AZ" => Some(0..4),
            "BA" => Some(0..3),
            "BE" => Some(0..3),
            "BG" => Some(0..4),
            "BH" => Some(0..4),
            "BI" => Some(0..5),
            "BR" => Some(0..8),
            "BY" => Some(0..4),
            "CH" => Some(0..5),
            "CR" => Some(0..4),
            "CY" => Some(0..3),
            "CZ" => Some(0..4),
            "DE" => Some(0..8),
            "DJ" => Some(0..5),
            "DK" => Some(0..4),
            "DO" => Some(0..4),
            "EE" => Some(0..2),
            "EG" => Some(0..4),
            "ES" => Some(0..4),
            "FI" => Some(0..3),
            "FK" => Some(0..2),
            "FO" => Some(0..4),
            "FR" => Some(0..5),
            "GB" => Some(0..4),
            "GE" => Some(0..2),
            "GI" => Some(0..4),
            "GL" => Some(0..4),
            "GR" => Some(0..3),
            "GT" => Some(0..4),
            "HR" => Some(0..7),
            "HU" => Some(0..3),
            "IE" => Some(0..4),
            "IL" => Some(0..3),
            "IQ" => Some(0..4),
            "IS" => Some(0..2),
            "IT" => Some(1..6),
            // Jordan has an incorrect bank identifier range in the registry.
            "JO" => Some(0..4),
            "KW" => Some(0..4),
            "KZ" => Some(0..3),
            "LB" => Some(0..4),
            "LC" => Some(0..4),
            "LI" => Some(0..5),
            "LT" => Some(0..5),
            "LU" => Some(0..3),
            "LV" => Some(0..4),
            "LY" => Some(0..3),
            "MC" => Some(0..5),
            "MD" => Some(0..2),
            "ME" => Some(0..3),
            "MK" => Some(0..3),
            "MN" => Some(0..4),
            "MR" => Some(0..5),
            "MT" => Some(0..4),
            "MU" => Some(0..6),
            "NI" => Some(0..4),
            "NL" => Some(0..4),
            "NO" => Some(0..4),
            "OM" => Some(0..3),
            "PL" => None,
            "PS" => Some(0..4),
            "PT" => Some(0..4),
            "QA" => Some(0..4),
            "RO" => Some(0..4),
            "RS" => Some(0..3),
            "RU" => Some(0..9),
            "SA" => Some(0..2),
            "SC" => Some(0..6),
            "SD" => Some(0..2),
            "SE" => Some(0..3),
            "SI" => Some(0..5),
            "SK" => Some(0..4),
            "SM" => Some(1..6),
            "SO" => Some(0..4),
            "ST" => Some(0..4),
            "SV" => Some(0..4),
            "TL" => Some(0..3),
            "TN" => Some(0..2),
            "TR" => Some(0..5),
            "UA" => Some(0..6),
            "VA" => Some(0..3),
            "VG" => Some(0..4),
            "XK" => Some(0..2),
            "YE" => Some(0..4),
            _ => panic!(
                "Unknown country! Please file an issue at \
                 https://github.com/ThomasdenH/iban_validate."
            ),
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
        #[allow(clippy::match_same_arms)] // For clarity, identical arms are not combined.
        match self.country_code() {
            "AD" => Some(4..8),
            "AE" => None,
            // The registry branch example ("1100") does not have the length as expected from the position range (3..8).
            // Assume the example is correct, see generation code for details.
            "AL" => Some(3..7),
            "AT" => None,
            "AZ" => None,
            "BA" => Some(3..6),
            "BE" => None,
            "BG" => Some(4..8),
            "BH" => None,
            "BI" => Some(5..10),
            "BR" => Some(8..13),
            "BY" => None,
            "CH" => None,
            "CR" => None,
            "CY" => Some(3..8),
            "CZ" => None,
            "DE" => None,
            "DJ" => Some(5..10),
            "DK" => None,
            "DO" => None,
            "EE" => None,
            "EG" => Some(4..8),
            "ES" => Some(4..8),
            "FI" => None,
            "FK" => None,
            "FO" => None,
            "FR" => None,
            "GB" => Some(4..10),
            "GE" => None,
            "GI" => None,
            "GL" => None,
            "GR" => Some(3..7),
            "GT" => None,
            "HR" => None,
            "HU" => Some(3..7),
            "IE" => Some(4..10),
            "IL" => Some(3..6),
            "IQ" => Some(4..7),
            "IS" => Some(2..4),
            "IT" => Some(6..11),
            // The registry doesn't provide an example.
            "JO" => Some(4..8),
            "KW" => None,
            "KZ" => None,
            "LB" => None,
            "LC" => None,
            "LI" => None,
            "LT" => None,
            "LU" => None,
            "LV" => None,
            "LY" => Some(3..6),
            "MC" => Some(5..10),
            "MD" => None,
            "ME" => None,
            "MK" => None,
            "MN" => None,
            "MR" => Some(5..10),
            "MT" => Some(4..9),
            "MU" => Some(6..8),
            "NI" => None,
            "NL" => None,
            "NO" => None,
            "OM" => None,
            "PL" => Some(0..8),
            "PS" => None,
            "PT" => Some(4..8),
            "QA" => None,
            "RO" => None,
            "RS" => None,
            "RU" => Some(9..14),
            "SA" => None,
            "SC" => Some(6..8),
            "SD" => None,
            "SE" => None,
            "SI" => None,
            "SK" => None,
            "SM" => Some(6..11),
            "SO" => Some(4..7),
            "ST" => Some(4..8),
            "SV" => None,
            "TL" => None,
            "TN" => Some(2..5),
            "TR" => None,
            "UA" => None,
            "VA" => None,
            "VG" => None,
            "XK" => Some(2..4),
            "YE" => Some(4..8),
            _ => panic!(
                "Unknown country! Please file an issue at \
                 https://github.com/ThomasdenH/iban_validate."
            ),
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
/// The impementation of [`Display`](fmt::Display) provides spaced formatting of the IBAN. Electronic
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
        use core::borrow::Borrow;
        use countries::{
            CharacterType::{A, C, N},
            Matchable,
        };
        #[allow(clippy::match_same_arms)] // For clarity, identical arms are not combined.
        (match base_iban.country_code() {
            "AD" => Some([(4, N), (4, N), (12, C)].borrow()),
            "AE" => Some([(3, N), (16, N)].borrow()),
            "AL" => Some([(8, N), (16, C)].borrow()),
            "AT" => Some([(5, N), (11, N)].borrow()),
            "AZ" => Some([(4, A), (20, C)].borrow()),
            "BA" => Some([(3, N), (3, N), (8, N), (2, N)].borrow()),
            "BE" => Some([(3, N), (7, N), (2, N)].borrow()),
            "BG" => Some([(4, A), (4, N), (2, N), (8, C)].borrow()),
            "BH" => Some([(4, A), (14, C)].borrow()),
            "BI" => Some([(5, N), (5, N), (11, N), (2, N)].borrow()),
            "BR" => Some([(8, N), (5, N), (10, N), (1, A), (1, C)].borrow()),
            "BY" => Some([(4, C), (4, N), (16, C)].borrow()),
            "CH" => Some([(5, N), (12, C)].borrow()),
            "CR" => Some([(4, N), (14, N)].borrow()),
            "CY" => Some([(3, N), (5, N), (16, C)].borrow()),
            "CZ" => Some([(4, N), (6, N), (10, N)].borrow()),
            "DE" => Some([(8, N), (10, N)].borrow()),
            "DJ" => Some([(5, N), (5, N), (11, N), (2, N)].borrow()),
            "DK" => Some([(4, N), (9, N), (1, N)].borrow()),
            "DO" => Some([(4, C), (20, N)].borrow()),
            "EE" => Some([(2, N), (2, N), (11, N), (1, N)].borrow()),
            "EG" => Some([(4, N), (4, N), (17, N)].borrow()),
            "ES" => Some([(4, N), (4, N), (1, N), (1, N), (10, N)].borrow()),
            "FI" => Some([(3, N), (11, N)].borrow()),
            "FK" => Some([(2, A), (12, N)].borrow()),
            "FO" => Some([(4, N), (9, N), (1, N)].borrow()),
            "FR" => Some([(5, N), (5, N), (11, C), (2, N)].borrow()),
            "GB" => Some([(4, A), (6, N), (8, N)].borrow()),
            "GE" => Some([(2, A), (16, N)].borrow()),
            "GI" => Some([(4, A), (15, C)].borrow()),
            "GL" => Some([(4, N), (9, N), (1, N)].borrow()),
            "GR" => Some([(3, N), (4, N), (16, C)].borrow()),
            "GT" => Some([(4, C), (20, C)].borrow()),
            "HR" => Some([(7, N), (10, N)].borrow()),
            "HU" => Some([(3, N), (4, N), (1, N), (15, N), (1, N)].borrow()),
            "IE" => Some([(4, A), (6, N), (8, N)].borrow()),
            "IL" => Some([(3, N), (3, N), (13, N)].borrow()),
            "IQ" => Some([(4, A), (3, N), (12, N)].borrow()),
            "IS" => Some([(4, N), (2, N), (6, N), (10, N)].borrow()),
            "IT" => Some([(1, A), (5, N), (5, N), (12, C)].borrow()),
            "JO" => Some([(4, A), (4, N), (18, C)].borrow()),
            "KW" => Some([(4, A), (22, C)].borrow()),
            "KZ" => Some([(3, N), (13, C)].borrow()),
            "LB" => Some([(4, N), (20, C)].borrow()),
            "LC" => Some([(4, A), (24, C)].borrow()),
            "LI" => Some([(5, N), (12, C)].borrow()),
            "LT" => Some([(5, N), (11, N)].borrow()),
            "LU" => Some([(3, N), (13, C)].borrow()),
            "LV" => Some([(4, A), (13, C)].borrow()),
            "LY" => Some([(3, N), (3, N), (15, N)].borrow()),
            "MC" => Some([(5, N), (5, N), (11, C), (2, N)].borrow()),
            "MD" => Some([(2, C), (18, C)].borrow()),
            "ME" => Some([(3, N), (13, N), (2, N)].borrow()),
            "MK" => Some([(3, N), (10, C), (2, N)].borrow()),
            "MN" => Some([(4, N), (12, N)].borrow()),
            "MR" => Some([(5, N), (5, N), (11, N), (2, N)].borrow()),
            "MT" => Some([(4, A), (5, N), (18, C)].borrow()),
            "MU" => Some([(4, A), (2, N), (2, N), (12, N), (3, N), (3, A)].borrow()),
            "NI" => Some([(4, A), (20, N)].borrow()),
            "NL" => Some([(4, A), (10, N)].borrow()),
            "NO" => Some([(4, N), (6, N), (1, N)].borrow()),
            "OM" => Some([(3, N), (16, C)].borrow()),
            "PL" => Some([(8, N), (16, N)].borrow()),
            "PS" => Some([(4, A), (21, C)].borrow()),
            "PT" => Some([(4, N), (4, N), (11, N), (2, N)].borrow()),
            "QA" => Some([(4, A), (21, C)].borrow()),
            "RO" => Some([(4, A), (16, C)].borrow()),
            "RS" => Some([(3, N), (13, N), (2, N)].borrow()),
            "RU" => Some([(9, N), (5, N), (15, C)].borrow()),
            "SA" => Some([(2, N), (18, C)].borrow()),
            "SC" => Some([(4, A), (2, N), (2, N), (16, N), (3, A)].borrow()),
            "SD" => Some([(2, N), (12, N)].borrow()),
            "SE" => Some([(3, N), (16, N), (1, N)].borrow()),
            "SI" => Some([(5, N), (8, N), (2, N)].borrow()),
            "SK" => Some([(4, N), (6, N), (10, N)].borrow()),
            "SM" => Some([(1, A), (5, N), (5, N), (12, C)].borrow()),
            "SO" => Some([(4, N), (3, N), (12, N)].borrow()),
            "ST" => Some([(4, N), (4, N), (11, N), (2, N)].borrow()),
            "SV" => Some([(4, A), (20, N)].borrow()),
            "TL" => Some([(3, N), (14, N), (2, N)].borrow()),
            "TN" => Some([(2, N), (3, N), (13, N), (2, N)].borrow()),
            "TR" => Some([(5, N), (1, N), (16, C)].borrow()),
            "UA" => Some([(6, N), (19, C)].borrow()),
            "VA" => Some([(3, N), (15, N)].borrow()),
            "VG" => Some([(4, A), (16, N)].borrow()),
            "XK" => Some([(4, N), (10, N), (2, N)].borrow()),
            "YE" => Some([(4, A), (4, N), (18, C)].borrow()),
            _ => None,
        })
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
