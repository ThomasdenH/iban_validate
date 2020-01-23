use crate::IbanLike;
use arrayvec::ArrayString;
use core::convert::TryFrom;
use core::fmt;
use core::str::FromStr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The size of a group of characters in the paper format.
const PAPER_GROUP_SIZE: usize = 4;

/// The maximum length an IBAN can be, according to the spec.
const MAX_IBAN_LEN: usize = 34;

/// Represents an IBAN that passed basic checks, but not necessarily the BBAN
/// validation. This corresponds to the validation as described in ISO 13616-1.
///
/// To be exact, the IBAN must start with two uppercase ASCII letters, followed
/// by two digits, followed by any number of digits and uppercase ASCII
/// letters. Additionally its checksum should be valid. It should either contain
/// no whitespace, or be in the paper format, where characters are in
/// space-separated groups of four.
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
            serializer.collect_str(self)
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
        for c in self.s.chars().enumerate().flat_map(|(i, c)| {
            // Add a space before a character if it is the start of a group of four.
            if i != 0 && i % PAPER_GROUP_SIZE == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(core::iter::once(c))
        }) {
            write!(f, "{}", c)?;
        }
        Ok(())
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
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ParseBaseIbanError {
    /// The string doesn't have the correct format to be an IBAN. This can be because it's too
    /// short, too long or because it contains unexpected characters at some location.
    InvalidFormat,
    /// The IBAN has an invalid structure.
    InvalidChecksum,
}

impl fmt::Display for ParseBaseIbanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParseBaseIbanError::InvalidFormat =>
                    "the string doesn't conform to the IBAN format",
                ParseBaseIbanError::InvalidChecksum => "the IBAN has an invalid checksum",
            }
        )
    }
}

#[cfg(feature = "std")]
use std::error::Error;

#[cfg(feature = "std")]
impl Error for ParseBaseIbanError {}

impl BaseIban {
    /// Compute the checksum for the address. The code that the string contains
    /// only valid characters: `'0'..='9'` and `'A'..='Z'`.
    fn validate_checksum(address: &str) -> bool {
        address
            .as_bytes()
            .iter()
            // Move the first four characters to the back
            .cycle()
            .skip(4)
            .take(address.len())
            // Calculate the checksum
            .fold(0, |acc, c| {
                // Convert '0'-'Z' to 0-35
                let digit = (*c as char).to_digit(36).expect(
                    "An address was supplied to compute_checksum with an invalid \
                     character. Please file an issue at \
                     https://github.com/ThomasdenH/iban_validate.",
                );
                // If the number consists of two digits, multiply by 100
                let multiplier = if digit > 9 { 100 } else { 10 };
                // Calculate modulo
                (acc * multiplier + digit) % 97
            })
            == 1 &&
            // Check digits with value 01 or 00 are invalid!
            &address[2..4] != "00" && 
            &address[2..4] != "01"
    }

    /// Parse a standardized IBAN string from an iterator. We iterate through
    /// bytes, not characters. When a character is not ASCII, the IBAN is
    /// automatically invalid.
    fn try_form_string_from_electronic<T>(
        mut chars: T,
    ) -> Result<ArrayString<[u8; MAX_IBAN_LEN]>, ParseBaseIbanError>
    where
        T: Iterator<Item = u8>,
    {
        let mut address_no_spaces = ArrayString::<[u8; MAX_IBAN_LEN]>::new();

        for _ in 0..2 {
            let c = match chars.next() {
                Some(c) if c.is_ascii_uppercase() => Ok(c),
                _ => Err(ParseBaseIbanError::InvalidFormat),
            }?;
            address_no_spaces
                .try_push(c as char)
                .map_err(|_| ParseBaseIbanError::InvalidFormat)?;
        }

        for _ in 0..2 {
            let c = match chars.next() {
                Some(c) if c.is_ascii_digit() => Ok(c),
                _ => Err(ParseBaseIbanError::InvalidFormat),
            }?;
            address_no_spaces
                .try_push(c as char)
                .map_err(|_| ParseBaseIbanError::InvalidFormat)?;
        }

        // The BBAN part can actually be both lower or upper case, but we
        // normalize it to uppercase here.
        for c in chars {
            if c.is_ascii_alphanumeric() {
                address_no_spaces
                    .try_push(c.to_ascii_uppercase() as char)
                    .map_err(|_| ParseBaseIbanError::InvalidFormat)?;
            } else {
                return Err(ParseBaseIbanError::InvalidFormat);
            }
        }

        Ok(address_no_spaces)
    }

    /// Parse a pretty print IBAN from a `str`.
    fn try_form_string_from_pretty_print(
        s: &str,
    ) -> Result<ArrayString<[u8; MAX_IBAN_LEN]>, ParseBaseIbanError> {
        // The pretty print format consists of a number of groups of four
        // characters, separated by a space.

        let bytes = s.as_bytes();

        // Check that every fifth character is a space...
        for (_, byte_at_space_position) in bytes.iter().enumerate().filter(|(i, _c)| i % 5 == 4) {
            if *byte_at_space_position != b' ' {
                return Err(ParseBaseIbanError::InvalidFormat);
            }
        }

        // ... except for the last group.
        if bytes.len() % 5 == 0 {
            return Err(ParseBaseIbanError::InvalidFormat);
        }

        // Now parse the remaining characters.
        BaseIban::try_form_string_from_electronic(
            bytes
                .iter()
                .enumerate()
                .filter_map(|(i, c)| if i % 5 != 4 { Some(c) } else { None })
                .copied(),
        )
    }
}

impl FromStr for BaseIban {
    type Err = ParseBaseIbanError;
    /// Parse a basic iban without taking the BBAN into consideration.
    ///
    /// # Errors
    /// If the string does not match the IBAN format or the checksum is
    /// invalid, an [`ParseBaseIbanError`](crate::ParseBaseIbanError) will be
    /// returned.
    fn from_str(address: &str) -> Result<Self, Self::Err> {
        let address_no_spaces =
            BaseIban::try_form_string_from_electronic(address.as_bytes().iter().copied())
                .or_else(|_| BaseIban::try_form_string_from_pretty_print(address))?;

        if !BaseIban::validate_checksum(&address_no_spaces) {
            return Err(ParseBaseIbanError::InvalidChecksum);
        }

        Ok(BaseIban {
            s: address_no_spaces,
        })
    }
}

impl<'a> TryFrom<&'a str> for BaseIban {
    type Error = ParseBaseIbanError;
    /// Parse a basic IBAN without taking the BBAN into consideration.
    ///
    /// # Errors
    /// If the string does not match the IBAN format or the checksum is
    /// invalid, an [`ParseBaseIbanError`](crate::ParseBaseIbanError) will be
    /// returned.
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
