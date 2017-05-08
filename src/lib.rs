/*!
This crate provides an easy way to validate an IBAN account number. To do this, it has two
functions, [`validate_iban`] which validates the IBAN format, and [`validate_iban_country`] which
validates the country specific format of the account number.

[`validate_iban`]: ./fn.validate_iban.html
[`validate_iban_country`]: ./fn.validate_iban_country.html

# Example
The following example does a complete validation of the account number:

```rust
use iban::*;

let account_number = "DE44500105175407324931";
let valid = validate_iban(account_number) &&
        validate_iban_country(account_number) == IbanCountryResult::Valid;

assert!(valid);
```

*/

#[cfg(test)]
extern crate spectral;
extern crate regex;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests;
mod countries;

use regex::Regex;
use countries::COUNTRY_FORMATS;

/// Validate an IBAN number. The validation will detect the following mistakes:
/// <ul>
///     <li>The length is four or less, or longer than 34.</li>
///     <li>The number contains characters other than A-Z or 0-9</li>
///     <li>A-Z is in place of 0-9 or vice versa</li>
///     <li>The checksum is invalid</li>
/// </ul>
/// If none of these apply, the function will return true, otherwise it will return false.
/// Note that this function will not check the country format. To validate the country code and
/// the BBAN format, you should also use [`validate_iban_country`].
///
/// [`validate_iban_country`]: ./fn.validate_iban_country.html
///
/// # Examples
/// ```rust
/// use iban::validate_iban;
///
/// // A valid address
/// assert_eq!(validate_iban("DE44500105175407324931"), true);
///
/// // An invalid address
/// assert_eq!(validate_iban("DE4450010234607324931"), false);
/// ```
pub fn validate_iban(address: &str) -> bool {
    return
        // Check the characters
        validate_characters(&address)
        // Check the checksum
        && compute_checksum(&address) == 1;
}

/// Checks whether all characters in this address are valid. Returns a true if all characters are
/// valid, false otherwise.
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
                .expect("An address was supplied to compute_checksum with an invalid character. \
                    Please file an issue at https://github.com/ThomasdenH/iban_validate.");
            // If the number consists of two digits, multiply by 100
            let multiplier = if digit > 9 { 100 } else { 10 };
            // Calculate modulo
            (acc * multiplier + digit) % 97
        }) as u8
}

/// The function [`validate_iban_country`] will return a variant of this enum.
///
/// [`validate_iban_country`]: ./fn.validate_iban_country.html
///
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum IbanCountryResult {
    /// The country was recognized and the code was valid
    Valid,
    /// The country was recognized and didn't fit the format
    Invalid,
    /// The country was not recognized
    CountryUnknown,
}

/// Validate the BBAN part of an IBAN account number. This function will return one of three
/// results:
/// <ul>
///     <li>If the country code is recognized and the address fits the country's format, it will
///         return [`IbanCountryResult::Valid`].</li>
///     <li>If the country code is recognized and the address does not fit the country BBAN format,
///         it will return [`IbanCountryResult::Invalid`].</li>
///     <li>If the country code is not recognized, it will return
///         [`IbanCountryResult::CountryUnknown`].</li>
/// </ul>
/// Note that this check is not a substitute for [`validate_iban`] or vice versa. This function
/// only checks the address country code and corresponding format. To verify whether the address
/// fits the IBAN specification, you should also call [`validate_iban`].
///
/// [`IbanCountryResult::Valid`]: ./enum.IbanCountryResult.html#variant.Valid
/// [`IbanCountryResult::Invalid`]: ./enum.IbanCountryResult.html#variant.Invalid
/// [`IbanCountryResult::CountryUnknown`]: ./enum.IbanCountryResult.html#variant.CountryUnknown
/// [`validate_iban`]: ./fn.validate_iban.html
///
/// # Examples
/// ```rust
/// use iban::validate_iban_country;
/// use iban::IbanCountryResult;
///
/// // A valid address format
/// assert_eq!(validate_iban_country("DE44500105175407324931"), IbanCountryResult::Valid);
///
/// // An invalid format
/// assert_eq!(validate_iban_country("DE44ABCDE5175407324931"), IbanCountryResult::Invalid);
///
/// // An unknown country
/// assert_eq!(validate_iban_country("ZZ44500105175407324931"), IbanCountryResult::CountryUnknown);
/// ```
pub fn validate_iban_country(address: &str) -> IbanCountryResult {
    let (country_code_address, address_remainder) = address.split_at(2);

    for &(country_code_pattern, country_regex) in COUNTRY_FORMATS.into_iter() {
        if country_code_pattern == country_code_address {
            // The country code matches
            let regex = Regex::new(country_regex)
                .expect("Could not compile regular expression. Please file an issue at \
                    https://github.com/ThomasdenH/iban_validate.");
            return if regex.is_match(address_remainder) {
                       IbanCountryResult::Valid
                   } else {
                       IbanCountryResult::Invalid
                   };
        }
    }
    IbanCountryResult::CountryUnknown
}
