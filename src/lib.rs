#[cfg(test)]
mod tests;

extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;

/// Validate an IBAN number. The validation will detect the following mistakes:
/// <ul>
///     <li>The length is four or less, or longer than 34.</li>
///     <li>The number contains characters other than A-Z or 0-9</li>
///     <li>A-Z is in place of 0-9 or vice versa</li>
///     <li>The checksum is invalid</li>
/// </ul>
/// If none of these apply, the function will return true, otherwise it will return false.
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
pub fn validate_iban<S: Into<String>>(address: S) -> bool {

    let address_string = address.into();

    return
        // Check the characters
        validate_characters(&address_string)
        // Check the checksum
        && compute_checksum(&address_string) == 1;
}

/// Checks whether all characters in this address are valid. Returns a true if all characters are
/// valid, false otherwise.
fn validate_characters(address: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[A-Z]{2}\d{2}[A-Z\d]{1,30}").unwrap();
    }
    RE.is_match(address)
}

/// This function computes the checksum of an address. The function assumes the string only
/// contains 0-9 and A-Z.
///
/// # Panics
/// If the address contains any characters other than 0-9 or A-Z, this function will panic.
fn compute_checksum(address: &String) -> u8 {
    let mut digits = Vec::new();

    // Move the first four characters to the back
    let (start, end) = address.split_at(4);
    let mut changed_order = String::new();
    changed_order.push_str(end);
    changed_order.push_str(start);

    // Convert the characters to digits
    for c in changed_order.chars() {
        match c {
            d @ '0'...'9' => digits.push(d.to_digit(10).unwrap()),
            a @ 'A'...'Z' => {
                let number = a.to_digit(36).unwrap();
                digits.push(number / 10);
                digits.push(number % 10);
            }
            _ => panic!("Invalid character in address"),
        }
    }

    // Validate the checksum
    digits.iter().fold(0, |acc, d| (acc * 10 + d) % 97) as u8
}
