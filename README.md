# iban_validate
[![](http://meritbadge.herokuapp.com/iban_validate)](https://crates.io/crates/iban_validate)
[![Build Status](https://travis-ci.org/ThomasdenH/iban_validate.svg?branch=master)](https://travis-ci.org/ThomasdenH/iban_validate)

This is a small crate that is able to validate an IBAN account number.

## Usage
To use this crate, just add it as an dependency:
    
    [dependencies]
    iban_validate = "0.1.1"

## Functionality
The crate has two functions that perform different tasks. See the [documentation](https://docs.rs/iban_validate/) for a
complete explanation including some examples.

### validate_iban()
This function validates the IBAN specification
- An address consists of 4 or less characters
- An address consists of 35 or more characters
- The address contains characters other than A-Z or 0-9
- The address doesn't start with two letters, followed by two numbers
- The checksum of the address is invalid

### validate_iban_country()
This function validates the BBAN country specific part of an IBAN address. It distinguishes between three different 
results:
- The country was recognized and the country format is valid
- The country was recognized and the country format is invalid
- The country was not recognized
