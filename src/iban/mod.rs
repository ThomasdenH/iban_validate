#[derive(Debug)]
pub struct Iban<'a> {
    address: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum IbanValidationResult {
    InvalidLength { l: usize },
    InvalidCharacter { i: usize, c: char },
    InvalidChecksum { s: u8 },
    Valid
}

impl<'a> Iban<'a> {
    /// Create a new IBAN address from a string. Note that this address might not be valid. This can
    /// be checked using validate().
    pub fn new<S: Into<&'a str>>(address: S) -> Iban<'a> {
        Iban {
            address: address.into()
        }
    }

    /// Validate an IBAN number. The validation process works in the following order:
    /// * If the length is incorrect, return InvalidLength
    /// * If the number contains invalid characters or are at the wrong position, return
    ///    InvalidCharacter
    /// * If the number has an invalid checksum, return InvalidChecksum
    /// * Otherwise, return Valid
    pub fn validate(&self) -> IbanValidationResult {

        // Check the length
        let length = self.address.len();
        if length < 4 || length > 34 {
            return IbanValidationResult::InvalidLength {l: length};
        }

        // Check the characters
        let char_result = self.validate_characters();
        if char_result != None {
            let (index, c) = char_result.unwrap();
            return IbanValidationResult::InvalidCharacter {i: index, c: c };
        }

        // Check the checksum
        match self.compute_checksum() {
            1 => IbanValidationResult::Valid,
            o => IbanValidationResult::InvalidChecksum { s: o }
        }
    }

    /// Checks whether all characters in this address are valid. Returns an option. If all
    /// characters are valid, None is returned. Otherwise, Some is returned with a tuple, containing
    /// the position and the invalid character.
    fn validate_characters(&self) -> Option<(usize, char)> {
        for (i, c) in self.address.char_indices() {
            match c {
                '0'...'9' => if i < 2 { return Some((i, c)); },
                'A'...'Z' => if i >= 2 && i < 4 { return Some((i, c)); },
                _ => return Some((i, c))
            }
        }
        None
    }

    /// This function computes the checksum of an address.
    ///
    /// # Panics
    /// If there are any invalid characters in the address, this function will panic.
    ///
    fn compute_checksum(&self) -> u8 {
        let mut digits = Vec::new();

        let (start, end) = self.address.split_at(4);
        let mut changed_order = String::new();
        changed_order.push_str(end);
        changed_order.push_str(start);

        for c in changed_order.chars() {
            match c {
                d @ '0'...'9' => digits.push(d.to_digit(10).unwrap()),
                a @ 'A'...'Z' => {
                    let number = a.to_digit(36).unwrap();
                    digits.push(number / 10);
                    digits.push(number % 10);
                },
                _ => panic!("Invalid character in address")
            }
        }

        digits.iter().fold(0, |acc, d| (acc * 10 + d) % 97) as u8
    }
}