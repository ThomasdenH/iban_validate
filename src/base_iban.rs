use crate::IbanLike;
use arrayvec::ArrayString;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// The maximum length an IBAN can be, according to the spec.
const MAX_IBAN_LEN: usize = 34;

/// The maximum length an IBAN can be including whitespace.
const MAX_IBAN_LEN_PRETTY: usize = MAX_IBAN_LEN + MAX_IBAN_LEN / 4;

/// The minimum length an IBAN can be, according to the spec.
const MIN_IBAN_LEN: usize = 5;

/// Represents an IBAN that passed basic checks, but not necessarily the BBAN validation.
/// To be exact, the IBAN must be of the correct length, start with two uppercase ASCII letters,
/// followed by two digits, followed by any number of digits and uppercase ASCII letters. Additionally
/// its checksum should be valid. It should either contain no whitespace, or every block of four
/// characters can be separated by a space.
///
/// Note that most useful methods are supplied by the trait [`IbanLike`](crate::IbanLike). The [`Display`](std::fmt::Display) trait provides pretty
/// print formatting.
///
/// # Examples
/// An example of parsing and using a correct IBAN:
/// ```rust
/// use iban::{BaseIban, IbanLike};
/// # use iban::ParseBaseIbanError;
///
/// let iban: BaseIban = "MR13 0002 0001 0100 0012 3456 753".parse()?;
/// assert_eq!(iban.electronic_str(), "MR1300020001010000123456753");
/// // The pretty print format
/// assert_eq!(iban.to_string(), "MR13 0002 0001 0100 0012 3456 753");
/// assert_eq!(iban.country_code(), "MR");
/// assert_eq!(iban.check_digits_str(), "13");
/// assert_eq!(iban.check_digits(), 13);
/// assert_eq!(iban.bban_unchecked(), "00020001010000123456753");
/// # Ok::<(), ParseBaseIbanError>(())
/// ```
///
/// An example of parsing invalid IBANs:
/// ```rust
/// use iban::{BaseIban, ParseBaseIbanError};
///
/// assert_eq!(
///     "MR$$".parse::<BaseIban>(),
///     Err(ParseBaseIbanError::InvalidFormat)
/// );
///
/// assert_eq!(
///     "MR0000020001010000123456754".parse::<BaseIban>(),
///     Err(ParseBaseIbanError::InvalidChecksum)
/// );
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BaseIban {
    /// The string representing the IBAN. The string contains only uppercase
    /// ASCII and digits and no whitespace. It starts with two letters followed
    /// by two digits.
    s: ArrayString<[u8; MAX_IBAN_LEN]>,
}

#[cfg(feature = "serde")]
impl Serialize for BaseIban {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_str(self.electronic_str())
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BaseIban {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IbanStringVisitor;
        use serde::de;

        impl<'vi> de::Visitor<'vi> for IbanStringVisitor {
            type Value = BaseIban;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "an IBAN string")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<BaseIban, E> {
                value.parse::<BaseIban>().map_err(E::custom)
            }
        }

        deserializer.deserialize_str(IbanStringVisitor)
    }
}

impl IbanLike for BaseIban {
    fn electronic_str(&self) -> &str {
        self.s.as_str()
    }
}

impl fmt::Debug for BaseIban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.s.fmt(f)
    }
}

impl fmt::Display for BaseIban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chars = self.electronic_str().chars().peekable();
        loop {
            for _ in 0..4 {
                if let Some(c) = chars.next() {
                    write!(f, "{}", c)?;
                } else {
                    return Ok(());
                }
            }
            if chars.peek().is_some() {
                write!(f, " ")?;
            }
        }
    }
}

/// Indicates that the string does not follow the basic IBAN rules.
///
/// # Example
/// An example of parsing invalid IBANs:
/// ```rust
/// use iban::{BaseIban, ParseBaseIbanError};
///
/// // Invalid formatting because the spaces are in the wrong places
/// assert_eq!(
///     "MR0 041 9".parse::<BaseIban>(),
///     Err(ParseBaseIbanError::InvalidFormat)
/// );
///
/// // This IBAN follows the correct basic format but has an invalid checksum
/// assert_eq!(
///     "MR00 0002 0001 0100 0012 3456 754".parse::<BaseIban>(),
///     Err(ParseBaseIbanError::InvalidChecksum)
/// );
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
pub enum ParseBaseIbanError {
    /// The string doesn't have the correct format to be an IBAN. This can be because it's too
    /// short, too long or because it contains unexpected characters at some location.
    #[error("the string doesn't conform to the IBAN format")]
    InvalidFormat,
    /// The IBAN has an invalid structure.
    #[error("the IBAN has an invalid checksum")]
    InvalidChecksum,
}

impl BaseIban {
    /// Compute the checksum for the address.
    fn compute_checksum(address: &str) -> u8 {
        address
            .chars()
            // Move the first four characters to the back
            .cycle()
            .skip(4)
            .take(address.len())
            // Calculate the checksum
            .fold(0, |acc, c| {
                // Convert '0'-'Z' to 0-35
                let digit = c.to_digit(36).expect(
                    "An address was supplied to compute_checksum with an invalid \
                     character. Please file an issue at \
                     https://github.com/ThomasdenH/iban_validate.",
                );
                // If the number consists of two digits, multiply by 100
                let multiplier = if digit > 9 { 100 } else { 10 };
                // Calculate modulo
                (acc * multiplier + digit) % 97
            }) as u8
    }

    /// Parse a standardized IBAN string from an iterator.
    fn try_form_string_from_electronic<T>(
        mut chars: T,
    ) -> Result<ArrayString<[u8; MAX_IBAN_LEN]>, ParseBaseIbanError>
    where
        T: Iterator<Item = char>,
    {
        let mut address_no_spaces = ArrayString::<[u8; MAX_IBAN_LEN]>::new();

        for _ in 0..2 {
            let c = match chars.next() {
                Some(c) if c.is_ascii_uppercase() => Ok(c),
                _ => Err(ParseBaseIbanError::InvalidFormat),
            }?;
            address_no_spaces.try_push(c).expect(
                "Could not push country code. Please create an issue at \
                 https://github.com/ThomasdenH/iban_validate.",
            );
        }

        for _ in 0..2 {
            let c = match chars.next() {
                Some(c) if c.is_ascii_digit() => Ok(c),
                _ => Err(ParseBaseIbanError::InvalidFormat),
            }?;
            address_no_spaces.try_push(c).expect(
                "Could not push country code. Please create an issue at \
                 https://github.com/ThomasdenH/iban_validate.",
            );
        }

        for c in chars {
            if c.is_ascii_digit() || c.is_ascii_uppercase() {
                address_no_spaces
                    .try_push(c)
                    .map_err(|_| ParseBaseIbanError::InvalidFormat)?;
            } else {
                return Err(ParseBaseIbanError::InvalidFormat);
            }
        }

        Ok(address_no_spaces)
    }

    /// PArse a pretty print IBAN from a `str`.
    fn try_form_string_from_pretty_print(
        s: &str,
    ) -> Result<ArrayString<[u8; MAX_IBAN_LEN]>, ParseBaseIbanError> {
        // Filter out correct whitespace and then pass through electronic parsing.
        s.chars()
            .enumerate()
            .find_map(|(i, c)| {
                if i % 5 == 4 && c != ' ' {
                    Some(Err(ParseBaseIbanError::InvalidFormat))
                } else {
                    None
                }
            })
            .unwrap_or(Ok(()))?;

        if s.ends_with(' ') {
            return Err(ParseBaseIbanError::InvalidFormat);
        }

        BaseIban::try_form_string_from_electronic(
            s.chars()
                .enumerate()
                .filter(|(i, _)| i % 5 != 4)
                .map(|(_, c)| c),
        )
    }
}

impl FromStr for BaseIban {
    type Err = ParseBaseIbanError;
    fn from_str(address: &str) -> Result<Self, Self::Err> {
        // Filter out obviously incorrect IBANS
        if address.len() < 5 || address.len() > MAX_IBAN_LEN_PRETTY {
            return Err(ParseBaseIbanError::InvalidFormat);
        }

        let address_no_spaces = BaseIban::try_form_string_from_electronic(address.chars())
            .or_else(|_| BaseIban::try_form_string_from_pretty_print(address))?;

        if address_no_spaces.len() < MIN_IBAN_LEN {
            return Err(ParseBaseIbanError::InvalidFormat);
        }

        if BaseIban::compute_checksum(&address_no_spaces) != 1 {
            return Err(ParseBaseIbanError::InvalidChecksum);
        }

        Ok(BaseIban {
            s: address_no_spaces,
        })
    }
}

impl<'a> TryFrom<&'a str> for BaseIban {
    type Error = ParseBaseIbanError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
