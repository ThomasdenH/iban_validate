# iban_validate
[![](http://meritbadge.herokuapp.com/iban_validate)](https://crates.io/crates/iban_validate)
[![Build Status](https://travis-ci.org/ThomasdenH/iban_validate.svg?branch=master)](https://travis-ci.org/ThomasdenH/iban_validate)

This is a small crate that is able to validate an IBAN account number.

## Usage
To use this crate, just add it as an dependency:
    
    [dependencies]
    iban_validate = "0.3.1"

## Functionality
The crate has two functions that perform different tasks. See the [documentation](https://docs.rs/iban_validate/) for a
complete explanation including some examples.

### [validate_iban()](https://docs.rs/iban_validate/0.3.1/iban/fn.validate_iban.html)
This function validates the IBAN specification. This means that an address will be regarded as valid unless:
- An address consists of 4 or less characters
- An address consists of 35 or more characters
- The address contains characters other than A-Z or 0-9
- The address does not start with two letters, followed by two numbers
- The checksum of the address is invalid

### [validate_iban_country()](https://docs.rs/iban_validate/0.3.1/iban/fn.validate_iban_country.html)
This function validates the BBAN country specific part of an IBAN address. It distinguishes between three different 
results:
- The country code was recognized and the country format is valid
- The country code was recognized and the country format is invalid
- The country code was not recognized

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.