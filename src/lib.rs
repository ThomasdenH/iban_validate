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

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/iban_validate/0.2.1")]

#[cfg(test)]
extern crate spectral;
extern crate regex;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests;
mod iban_standard;
mod iban_countries;

pub use iban_standard::validate_iban;

pub use iban_countries::IbanCountryResult;
pub use iban_countries::validate_iban_country;
