# iban_validate
[![](http://meritbadge.herokuapp.com/iban_validate)](https://crates.io/crates/iban_validate)
[![Build Status](https://travis-ci.org/ThomasdenH/iban_check.svg?branch=master)](https://travis-ci.org/ThomasdenH/iban_check)

This is a small crate that is able to validate an IBAN account number. It will find the following mistakes:
- An address consists of 4 or less characters
- An address consists of 35 or more characters
- The address contains characters other than A-Z or 0-9
- The address doesn't start with two letters, followed by two numbers
- The checksum of the address is invalid
