use crate::IbanLike;
use arrayvec::ArrayString;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// The maximum length an IBAN can be, according to the spec.
const MAX_IBAN_LEN: usize = 24;

/// Represents an IBAN that passed basic checks, but not necessarily the BBAN validation.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BaseIban {
    /// The string representing the IBAN. The string contains only uppercase
    /// ASCII and digits and no whitespace. It starts with two letters followed
    /// by two digits.
    s: ArrayString<[u8; MAX_IBAN_LEN]>,
}

impl IbanLike for BaseIban {
    fn electronic_str(&self) -> &str {
        self.s.as_str()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ParseBaseIbanError {
    InvalidCharacter(char),
    TooLong,
    WrongStructure,
    InvalidChecksum,
}

impl fmt::Display for ParseBaseIbanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseBaseIbanError::InvalidCharacter(c) => {
                write!(f, "The IBAN contains an invalid character: {}", c)
            }
            ParseBaseIbanError::TooLong => write!(f, "The IBAN is too long."),
            ParseBaseIbanError::WrongStructure => write!(f, "The IBAN has an invalid structure."),
            ParseBaseIbanError::InvalidChecksum => write!(f, "The IBAN has an invalid checksum."),
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
                return Err(ParseBaseIbanError::TooLong);
            }
        }

        if !BaseIban::validate_iban_characters(&address_no_spaces) {
            return Err(ParseBaseIbanError::WrongStructure);
        }
        if BaseIban::compute_checksum(&address_no_spaces) == 1 {
            return Err(ParseBaseIbanError::InvalidChecksum);
        }

        Ok(BaseIban {
            s: address_no_spaces,
        })
    }
}
