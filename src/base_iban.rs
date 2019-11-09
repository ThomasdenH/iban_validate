use crate::IbanLike;
use arrayvec::ArrayString;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// The maximum length an IBAN can be, according to the spec.
const MAX_IBAN_LEN: usize = 34;
/// The minimum length an IBAN can be, according to the spec.
const MIN_IBAN_LEN: usize = 5;

/// Represents an IBAN that passed basic checks, but not necessarily the BBAN validation.
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
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ParseBaseIbanError {
    /// The string contains an invalid character.
    InvalidCharacter(char),
    /// The string, excluding whitespace, has an invalid length.
    InvalidLength,
    /// The string contains an invalid characters in a wrong location.
    InvalidStructure,
    /// The IBAN has an invalid structure.
    InvalidChecksum,
}

impl fmt::Display for ParseBaseIbanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseBaseIbanError::InvalidCharacter(c) => {
                write!(f, "the IBAN contains an invalid character: {}", c)
            }
            ParseBaseIbanError::InvalidLength => write!(f, "the IBAN has an invalid length"),
            ParseBaseIbanError::InvalidStructure => write!(f, "the IBAN has an invalid structure"),
            ParseBaseIbanError::InvalidChecksum => write!(f, "the IBAN has an invalid checksum"),
        }
    }
}

impl Error for ParseBaseIbanError {}

impl BaseIban {
    /// Validates whether the first four characters are of the correct character set.
    /// All characters should already be either ASCII uppercase or digits.
    fn validate_iban_characters(address: &str) -> bool {
        address[0..2].chars().all(|c| c.is_ascii_uppercase())
            && address[2..4].chars().all(|c| c.is_ascii_digit())
    }

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
}

impl FromStr for BaseIban {
    type Err = ParseBaseIbanError;
    fn from_str(address: &str) -> Result<Self, Self::Err> {
        let mut address_no_spaces = ArrayString::<[u8; MAX_IBAN_LEN]>::new();
        for c in address.chars() {
            // Skip whitespace
            if c.is_ascii_whitespace() {
                continue;
            // Otherwise accept only valid characters
            } else if !c.is_ascii_digit() && !c.is_ascii_uppercase() {
                return Err(ParseBaseIbanError::InvalidCharacter(c));
            // Append the character and return an error when the address is too long.
            } else if address_no_spaces.try_push(c).is_err() {
                return Err(ParseBaseIbanError::InvalidLength);
            }
        }

        if address_no_spaces.len() < MIN_IBAN_LEN {
            return Err(ParseBaseIbanError::InvalidLength);
        }

        if !BaseIban::validate_iban_characters(&address_no_spaces) {
            return Err(ParseBaseIbanError::InvalidStructure);
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
