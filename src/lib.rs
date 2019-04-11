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
//! assert_eq!(account.validate_bban(), BbanResult::Valid);
//! assert_eq!(account.get_country_code(), "DE");
//! assert_eq!(account.get_check_digits(), 44);
//! assert_eq!(account.get_bban(), "500105175407324931");
//! assert_eq!(account.format_electronic(), "DE44500105175407324931");
//! assert_eq!(account.format_print(), "DE44 5001 0517 5407 3249 31");
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

use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::ops;
use std::str;

use crate::countries::RE_ADDRESS_REMAINDER;
use crate::countries::RE_COUNTRY_CODE;

pub use crate::countries::BbanResult;

mod countries;
#[cfg(test)]
mod tests;

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
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Iban(String);

/// The result after using [`parse()`] or [`from_str()`] on an invalid IBAN. It indicates that the
/// string does not follow the IBAN specification.
///
/// # Examples
///
/// ```rust
/// use iban::Iban;
/// use iban::ParseIbanError;
///
/// // Too short
/// assert!(match "AA32".parse::<Iban>() {
///     Err(ParseIbanError{..}) => true,
///     _ => false
/// });
/// ```
///
/// [`parse()`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
/// [`from_str()`]: /struct.Iban.html#method.from_str
///
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParseIbanError {
    _private: (),
}

static PARSE_IBAN_ERROR_DESCRIPTION: &'static str = "account number does not follow the IBAN \
                                                     format";

impl fmt::Display for ParseIbanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PARSE_IBAN_ERROR_DESCRIPTION.fmt(f)
    }
}

impl Error for ParseIbanError {
    fn description(&self) -> &str {
        PARSE_IBAN_ERROR_DESCRIPTION
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Iban {
    /// Returns the country code of an IBAN. The country code consists of the first two letters of
    /// an address.
    ///
    /// # Examples
    /// ```rust
    /// use iban::Iban;
    ///
    /// assert_eq!("AD1200012030200359100100".parse::<Iban>()?
    ///     .get_country_code(),
    ///     "AD"
    /// );
    /// # Ok::<(), iban::ParseIbanError>(())
    /// ```
    pub fn get_country_code(&self) -> &str {
        let (country_code, _) = self.split_at(2);
        country_code
    }

    /// Returns the check digits of an IBAN. These are the third and the fourth characters of an
    /// address.
    ///
    /// # Examples
    /// ```rust
    /// use iban::Iban;
    ///
    /// assert_eq!("AD1200012030200359100100".parse::<Iban>()?
    ///     .get_check_digits(),
    ///     12
    /// );
    /// # Ok::<(), iban::ParseIbanError>(())
    /// ```
    pub fn get_check_digits(&self) -> u8 {
        let (_, after_country_code) = self.split_at(2);
        let (check_digits, _) = after_country_code.split_at(2);
        check_digits.parse().expect(
            "Could not parse check digits. Please create an issue at \
             https://github.com/ThomasdenH/iban_validate.",
        )
    }

    /// Returns the BBAN part of an IBAN. It consists of all characters after the country code and
    /// check digits and is country specific. To validate that it follows the correct country
    /// format, use [`validate_bban()`].
    ///
    /// # Examples
    /// ```rust
    /// use iban::Iban;
    ///
    /// assert_eq!("AD1200012030200359100100".parse::<Iban>()?
    ///     .get_bban(),
    ///     "00012030200359100100"
    /// );
    /// # Ok::<(), iban::ParseIbanError>(())
    /// ```
    ///
    /// [`validate_bban()`]: ./struct.Iban.html#method.validate_bban
    ///
    pub fn get_bban(&self) -> &str {
        let (_, bban) = self.split_at(4);
        bban
    }

    /// Validates the BBAN part of an IBAN account number. It returns one of three results:
    ///
    /// - If the country code is recognized and the address fits the country's format, it will
    ///   return [`BbanResult::Valid`].
    /// - If the country code is recognized and the address does not fit the country BBAN format,
    ///   it will return [`BbanResult::Invalid`].
    /// - If the country code is not recognized, it will return
    ///   [`BbanResult::CountryUnknown`].
    ///
    /// [`BbanResult::Valid`]: ./enum.BbanResult.html#variant.Valid
    /// [`BbanResult::Invalid`]: ./enum.BbanResult.html#variant.Invalid
    /// [`BbanResult::CountryUnknown`]: ./enum.BbanResult.html#variant.CountryUnknown
    ///
    /// # Examples
    ///
    /// ```rust
    /// use iban::Iban;
    /// use iban::BbanResult;
    ///
    /// // A valid BBAN
    /// let iban1 = "DE44500105175407324931".parse::<Iban>()?;
    /// assert_eq!(iban1.validate_bban(), BbanResult::Valid);
    ///
    /// // An invalid BBAN
    /// let iban2: Iban = "BA6312900794010284AC".parse()?;
    /// assert_eq!(iban2.validate_bban(), BbanResult::Invalid);
    ///
    /// // An unknown country
    /// let iban3: Iban = "ZZ07273912631298461".parse()?;
    /// assert_eq!(iban3.validate_bban(), BbanResult::CountryUnknown);
    /// # Ok::<(), iban::ParseIbanError>(())
    /// ```
    pub fn validate_bban(&self) -> BbanResult {
        let country_match = RE_COUNTRY_CODE
            .matches(self.get_country_code())
            .iter()
            .next();

        if let Some(country_index) = country_match {
            let address_match = RE_ADDRESS_REMAINDER
                .matches(self.get_bban())
                .iter()
                .find(|&address_index| address_index == country_index);

            if address_match.is_some() {
                BbanResult::Valid
            } else {
                BbanResult::Invalid
            }
        } else {
            BbanResult::CountryUnknown
        }
    }

    /// Returns the electronic format of an IBAN. This a simple string representation without any
    /// whitespace. To get the pretty print format, see [`format_print()`] or make use of the
    /// implementation of the [`Display`] trait.
    ///
    /// [`format_print()`]: ./struct.Iban.html#method.format_print
    /// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
    ///
    /// # Examples
    /// ```rust
    /// let iban = "DO28 BAGR 0000 0001 2124 5361 1324".parse::<iban::Iban>()?;
    /// assert_eq!(iban.format_electronic(), "DO28BAGR00000001212453611324");
    /// # Ok::<(), iban::ParseIbanError>(())
    /// ```
    ///
    pub fn format_electronic(&self) -> String {
        self.0.clone()
    }

    /// Returns the pretty print format of an IBAN. This is a string representation where every
    /// four characters are seperated by a space. The [`Display`] trait also produces strings
    /// formatted in this way. To get an electronic representation without spaces, use
    /// [`format_electronic()`].
    ///
    /// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
    /// [`format_electronic()`]: ./struct.Iban.html#method.format_electronic
    ///
    /// # Examples
    /// ```rust
    /// let iban = "DO28BAGR00000001212453611324".parse::<iban::Iban>()?;
    /// assert_eq!(iban.format_print(), "DO28 BAGR 0000 0001 2124 5361 1324");
    /// # Ok::<(), iban::ParseIbanError>(())
    /// ```
    ///
    pub fn format_print(&self) -> String {
        let mut spaced_string = String::new();
        let mut char_iter = self.0.chars();
        for i in 0..(self.0.len() + (self.0.len() - 1) / 4) {
            spaced_string.push(if i % 5 == 4 {
                ' '
            } else {
                // Since we iterate over the correct length, this should always give Ok()
                char_iter.next().expect(
                    "Unexpected None found while formatting. Please file an issue at \
                     https://github.com/ThomasdenH/iban_validate.",
                )
            });
        }
        spaced_string
    }

    /// Checks whether all characters in this address are valid. Returns a true if all characters
    /// are valid, false otherwise.
    fn validate_characters(address: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[A-Z]{2}\d{2}[A-Z\d]{1,30}$").expect(
                "Could not compile regular expression. Please file an issue at \
                 https://github.com/ThomasdenH/iban_validate."
            );
        }
        RE.is_match(address)
    }

    /// This function computes the checksum of an address. The function assumes the string only
    /// contains 0-9 and A-Z.
    ///
    /// # Panics
    /// If the address contains any characters other than 0-9 or A-Z, this function will panic.
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

impl str::FromStr for Iban {
    type Err = ParseIbanError;

    /// Parses an IBAN. If the conversion succeeds, the function will return [`Ok`],
    /// containing the parsed [`Iban`] struct.
    ///
    /// # Examples
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// // Explicit usage
    /// let address1 = iban::Iban::from_str("DE44500105175407324931")?;
    /// let address2 = iban::Iban::from_str("DE44 5001 0517 5407 3249 31")?;
    ///
    /// // Implicit usage
    /// let address3 = "DE44500105175407324931".parse::<iban::Iban>()?;
    /// # Ok::<(), iban::ParseIbanError>(())
    /// ```
    ///
    /// # Errors
    /// The conversion can fail if the input is not a valid IBAN. The function will check that none
    /// of the following apply:
    ///
    /// - The length is four or less, or longer than 34.
    /// - The number contains characters other than A-Z or 0-9
    /// - A-Z is in place of 0-9 or vice versa
    /// - The checksum is invalid
    ///
    /// Note that this function will not check the country format. To validate the country
    /// code and the BBAN format, you should also use [`validate_bban()`].
    ///
    /// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Iban`]: ./struct.Iban.html
    /// [`validate_bban()`]: ./struct.Iban.html#method.validate_bban
    ///
    fn from_str(address: &str) -> Result<Self, Self::Err> {
        // Remove the spaces
        let address_no_spaces = address.replace(" ", "");

        // Validate the Iban
        if Iban::validate_characters(&address_no_spaces)
            && Iban::compute_checksum(&address_no_spaces) == 1
        {
            Ok(Iban(address_no_spaces))
        } else {
            Err(ParseIbanError { _private: () })
        }
    }
}

impl fmt::Display for Iban {
    /// Displays an IBAN in the pretty print format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let account: iban::Iban = "DE44500105175407324931".parse()?;
    /// assert_eq!(format!("{}", account), "DE44 5001 0517 5407 3249 31");
    /// # Ok::<(), iban::ParseIbanError>(())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_print().fmt(f)
    }
}

impl ops::Deref for Iban {
    type Target = str;

    fn deref(&self) -> &str {
        self.0.as_str()
    }
}
