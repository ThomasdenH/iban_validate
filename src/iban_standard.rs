//! This module contains the code for the validation of the IBAN standard. This means matching
//! against a general regular expression and calculating the ISO 7064 checksum.

extern crate regex;
use regex::Regex;

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
