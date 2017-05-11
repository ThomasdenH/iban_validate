//! This crate provides an easy way to validate an IBAN. To do this, you can use the function
//! [`parse()`]. If you want to check whether the address has a valid BBAN, you can then use
//! [`validate_bban()`].
//!
//! # Example
//! The following example does a full validation of the IBAN and BBAN format:
//!
//! ```rust
//! let account_number = "DE44500105175407324931";
//! let valid = match account_number.parse::<iban::Iban>() {
//!     Ok(account) => account.validate_bban() == iban::BbanResult::Valid,
//!     Err(_) => false
//! };
//! assert!(valid);
//! ```
//!
//! [`parse()`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
//! [`validate_bban()`]: struct.Iban.html#method.validate_bban

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/iban_validate/0.3.1")]

extern crate regex;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate spectral;

#[cfg(test)]
mod tests;
mod iban_countries;

pub use iban_countries::BbanResult;

use std::str;
use std::fmt;
use std::ops;
use regex::Regex;

use iban_countries::RE_COUNTRY_CODE;
use iban_countries::RE_ADDRESS_REMAINDER;

/// Iban represents an IBAN (International Bank Account Number). To obtain it, make use of the
/// [`parse()`] function. This will make sure the string is formatted correctly.
///
/// # Examples
/// ```rust
/// use iban::Iban;
///
/// let address = "KZ86125KZT5004100100".parse::<Iban>().expect("Could not parse IBAN.");
/// ```
///
/// [`parse()`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Iban(String);

/// Indicates that the string does not follow the IBAN specification.
///
/// # Examples
///
/// ```rust
/// use iban::Iban;
/// use iban::ParseIbanError;
///
/// // Too short
/// let address_result = "AA32".parse::<Iban>();
///
/// assert!(match address_result {
///     Err(ParseIbanError{..}) => true,
///     _ => false
/// });
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParseIbanError {
    _private: (),
}

impl Iban {
    /// Returns the country code of an Iban.
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!("AD1200012030200359100100".parse::<iban::Iban>()
    ///     .expect("Could not parse IBAN.")
    ///     .get_country_code(),
    ///     "AD"
    /// );
    /// ```
    pub fn get_country_code(&self) -> &str {
        let (country_code, _) = self.split_at(2);
        country_code
    }

    /// Returns the check digits of an IBAN.
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!("AD1200012030200359100100".parse::<iban::Iban>()
    ///     .expect("Could not parse IBAN.")
    ///     .get_check_digits(),
    ///     "12"
    /// );
    /// ```
    pub fn get_check_digits(&self) -> &str {
        let (_, after_country_code) = self.split_at(2);
        let (check_digits, _) = after_country_code.split_at(2);
        check_digits
    }

    /// Returns the BBAN part of an IBAN.
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!("AD1200012030200359100100".parse::<iban::Iban>()
    ///     .expect("Could not parse IBAN.")
    ///     .get_bban(),
    ///     "00012030200359100100"
    /// );
    /// ```
    pub fn get_bban(&self) -> &str {
        let (_, bban) = self.split_at(4);
        bban
    }

    /// Validate the BBAN part of an IBAN account number.
    ///
    /// This function will return one of three results:
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
    /// let iban1: Iban = "DE44500105175407324931".parse().unwrap();
    /// assert_eq!(iban1.validate_bban(), BbanResult::Valid);
    ///
    /// // An invalid BBAN
    /// let iban2: Iban = "BA6312900794010284AC".parse().unwrap();
    /// assert_eq!(iban2.validate_bban(), BbanResult::Invalid);
    ///
    /// // An unknown country
    /// let iban3: Iban = "ZZ07273912631298461".parse().unwrap();
    /// assert_eq!(iban3.validate_bban(), BbanResult::CountryUnknown);
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

    /// Checks whether all characters in this address are valid. Returns a true if all characters
    /// are valid, false otherwise.
    fn validate_characters(address: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[A-Z]{2}\d{2}[A-Z\d]{1,30}$")
                .expect("Could not compile regular expression. Please file an issue at \
                    https://github.com/ThomasdenH/iban_validate.");
        }
        RE.is_match(address)
    }

    /// This function computes the checksum of an address. The function assumes the string only
    /// contains 0-9 and A-Z.
    ///
    /// # Panics
    /// If the address contains any characters other than 0-9 or A-Z, this function will panic.
    fn compute_checksum(address: &str) -> u8 {
        address.chars()
            // Move the first four characters to the back
            .cycle()
            .skip(4)
            .take(address.len())
            // Calculate the checksum
            .fold(0, |acc, c| {
                // Convert '0'-'Z' to 0-35
                let digit = c.to_digit(36)
                    .expect("An address was supplied to compute_checksum with an invalid \
                    character. Please file an issue at \
                    https://github.com/ThomasdenH/iban_validate.");
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
    /// let address1 = iban::Iban::from_str("DE44500105175407324931")
    ///     .expect("Could not parse IBAN!");
    ///
    /// let address2 = "DE44500105175407324931".parse::<iban::Iban>()
    ///     .expect("Could not parse IBAN!");
    /// ```
    ///
    /// # Errors
    /// The conversion can fail if the input is not a valid IBAN. This function will check that
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
    /// [`validate_bban`]: ./struct.Iban.html#method.validate_bban
    ///
    fn from_str(address: &str) -> Result<Self, Self::Err> {
        if Iban::validate_characters(&address) && Iban::compute_checksum(&address) == 1 {
            Ok(Iban(address.to_string()))
        } else {
            if Iban::validate_characters(&address) {
                println!("{}: {}", address, Iban::compute_checksum(&address));
            }
            Err(ParseIbanError { _private: () })
        }
    }
}

impl fmt::Display for Iban {
    /// Display an IBAN.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let account: iban::Iban = "DE44500105175407324931".parse()
    ///     .expect("Could not parse IBAN!");
    ///
    /// assert_eq!(format!("{}", account), "DE44500105175407324931");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ops::Deref for Iban {
    type Target = str;

    /// Deref an Iban to use all methods of &str as well.
    fn deref(&self) -> &str {
        self.0.as_str()
    }
}
