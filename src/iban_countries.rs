//! This module contains the code for the validation of the countries in the Swift IBAN registry. It
//! checks if the country code is recognized and then tries to match the regular expression.

extern crate regex;

use regex::Regex;

static COUNTRY_FORMATS: [(&'static str, &'static str); 75] =
    [("AD", r"^\d{10}[A-Z\d]{12}$"),
     ("AE", r"^\d{21}$"),
     ("AL", r"^\d{10}[A-Z\d]{16}$"),
     ("AT", r"^\d{18}$"),
     ("AZ", r"^\d{2}[A-Z]{4}[A-Z\d]{20}$"),
     ("BA", r"^\d{18}$"),
     ("BE", r"^\d{14}$"),
     ("BG", r"^\d{2}[A-Z]{4}\d{6}[A-Z\d]{8}$"),
     ("BH", r"^\d{2}[A-Z]{4}[A-Z\d]{14}$"),
     ("BR", r"^\d{25}[A-Z]{1}[A-Z\d]{1}$"),
     ("BY", r"^\d{2}[A-Z\d]{4}\d{4}[A-Z\d]{16}$"),
     ("CH", r"^\d{7}[A-Z\d]{12}$"),
     ("CR", r"^\d{20}$"),
     ("CY", r"^\d{10}[A-Z\d]{16}$"),
     ("CZ", r"^\d{22}$"),
     ("DE", r"^\d{20}$"),
     ("DK", r"^\d{16}$"),
     ("DO", r"^\d{2}[A-Z\d]{4}\d{20}$"),
     ("EE", r"^\d{18}$"),
     ("ES", r"^\d{22}$"),
     ("FI", r"^\d{16}$"),
     ("FO", r"^\d{16}$"),
     ("FR", r"^\d{12}[A-Z\d]{11}\d{2}$"),
     ("GB", r"^\d{2}[A-Z]{4}\d{14}$"),
     ("GE", r"^\d{2}[A-Z]{2}\d{16}$"),
     ("GI", r"^\d{2}[A-Z]{4}[A-Z\d]{15}$"),
     ("GL", r"^\d{16}$"),
     ("GR", r"^\d{9}[A-Z\d]{16}$"),
     ("GT", r"^\d{2}[A-Z\d]{24}$"),
     ("HR", r"^\d{19}$"),
     ("HU", r"^\d{26}$"),
     ("IE", r"^\d{2}[A-Z]{4}\d{14}$"),
     ("IL", r"^\d{21}$"),
     ("IQ", r"^\d{2}[A-Z]{4}\d{15}$"),
     ("IS", r"^\d{24}$"),
     ("IT", r"^\d{2}[A-Z]{1}\d{10}[A-Z\d]{12}$"),
     ("JO", r"^\d{2}[A-Z]{4}\d{4}[A-Z\d]{18}$"),
     ("KW", r"^\d{2}[A-Z]{4}[A-Z\d]{22}$"),
     ("KZ", r"^\d{5}[A-Z\d]{13}$"),
     ("LB", r"^\d{6}[A-Z\d]{20}$"),
     ("LC", r"^\d{2}[A-Z]{4}[A-Z\d]{24}$"),
     ("LI", r"^\d{7}[A-Z\d]{12}$"),
     ("LT", r"^\d{18}$"),
     ("LU", r"^\d{5}[A-Z\d]{13}$"),
     ("LV", r"^\d{2}[A-Z]{4}[A-Z\d]{13}$"),
     ("MC", r"^\d{12}[A-Z\d]{11}\d{2}$"),
     ("MD", r"^\d{2}[A-Z\d]{20}$"),
     ("ME", r"^\d{20}$"),
     ("MK", r"^\d{5}[A-Z\d]{10}\d{2}$"),
     ("MR", r"^\d{25}$"),
     ("MT", r"^\d{2}[A-Z]{4}\d{5}[A-Z\d]{18}$"),
     ("MU", r"^\d{2}[A-Z]{4}\d{19}[A-Z]{3}$"),
     ("NL", r"^\d{2}[A-Z]{4}\d{10}$"),
     ("NO", r"^\d{13}$"),
     ("PK", r"^\d{2}[A-Z]{4}[A-Z\d]{16}$"),
     ("PL", r"^\d{26}$"),
     ("PS", r"^\d{2}[A-Z]{4}[A-Z\d]{21}$"),
     ("PT", r"^\d{23}$"),
     ("QA", r"^\d{2}[A-Z]{4}[A-Z\d]{21}$"),
     ("RO", r"^\d{2}[A-z]{4}[A-Z\d]{16}$"),
     ("RS", r"^\d{20}$"),
     ("SA", r"^\d{4}[A-Z\d]{18}$"),
     ("SC", r"^\d{2}[A-Z]{4}\d{20}[A-Z]{3}$"),
     ("SE", r"^\d{22}$"),
     ("SI", r"^\d{17}$"),
     ("SK", r"^\d{22}$"),
     ("SM", r"^\d{2}[A-Z]{1}\d{10}[A-Z\d]{12}$"),
     ("ST", r"^\d{23}$"),
     ("SV", r"^\d{2}[A-Z]{4}\d{20}$"),
     ("TL", r"^\d{21}$"),
     ("TN", r"^\d{22}$"),
     ("TR", r"^\d{8}[A-Z\d]{16}$"),
     ("UA", r"^\d{8}[A-Z\d]{19}$"),
     ("VG", r"^\d{2}[A-Z]{4}\d{16}$"),
     ("XK", r"^\d{18}$")];

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
