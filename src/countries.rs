//! This module contains the code for the validation of the countries in the Swift IBAN registry. It
//! checks if the country code is recognized and then tries to match the regular expression.

use lazy_static::lazy_static;
use regex;
use regex::{RegexSet, RegexSetBuilder};

static COUNTRY_FORMATS: [(&str, &str); 76] = [
    ("AD", r"^\d{8}[A-Z\d]{12}$"),
    ("AE", r"^\d{19}$"),
    ("AL", r"^\d{8}[A-Z\d]{16}$"),
    ("AT", r"^\d{16}$"),
    ("AZ", r"^[A-Z]{4}[A-Z\d]{20}$"),
    ("BA", r"^\d{16}$"),
    ("BE", r"^\d{12}$"),
    ("BG", r"^[A-Z]{4}\d{6}[A-Z\d]{8}$"),
    ("BH", r"^[A-Z]{4}[A-Z\d]{14}$"),
    ("BR", r"^\d{23}[A-Z]{1}[A-Z\d]{1}$"),
    ("BY", r"^[A-Z\d]{4}\d{4}[A-Z\d]{16}$"),
    ("CH", r"^\d{5}[A-Z\d]{12}$"),
    ("CR", r"^\d{18}$"),
    ("CY", r"^\d{8}[A-Z\d]{16}$"),
    ("CZ", r"^\d{20}$"),
    ("DE", r"^\d{18}$"),
    ("DK", r"^\d{14}$"),
    ("DO", r"^[A-Z\d]{4}\d{20}$"),
    ("EE", r"^\d{16}$"),
    ("ES", r"^\d{20}$"),
    ("FI", r"^\d{14}$"),
    ("FO", r"^\d{14}$"),
    ("FR", r"^\d{10}[A-Z\d]{11}\d{2}$"),
    ("GB", r"^[A-Z]{4}\d{14}$"),
    ("GE", r"^[A-Z]{2}\d{16}$"),
    ("GI", r"^[A-Z]{4}[A-Z\d]{15}$"),
    ("GL", r"^\d{14}$"),
    ("GR", r"^\d{7}[A-Z\d]{16}$"),
    ("GT", r"^[A-Z\d]{24}$"),
    ("HR", r"^\d{17}$"),
    ("HU", r"^\d{24}$"),
    ("IE", r"^[A-Z]{4}\d{14}$"),
    ("IL", r"^\d{19}$"),
    ("IQ", r"^[A-Z]{4}\d{15}$"),
    ("IS", r"^\d{22}$"),
    ("IT", r"^[A-Z]{1}\d{10}[A-Z\d]{12}$"),
    ("JO", r"^[A-Z]{4}\d{4}[A-Z\d]{18}$"),
    ("KW", r"^[A-Z]{4}[A-Z\d]{22}$"),
    ("KZ", r"^\d{3}[A-Z\d]{13}$"),
    ("LB", r"^\d{4}[A-Z\d]{20}$"),
    ("LC", r"^[A-Z]{4}[A-Z\d]{24}$"),
    ("LI", r"^\d{5}[A-Z\d]{12}$"),
    ("LT", r"^\d{16}$"),
    ("LU", r"^\d{3}[A-Z\d]{13}$"),
    ("LV", r"^[A-Z]{4}[A-Z\d]{13}$"),
    ("MC", r"^\d{10}[A-Z\d]{11}\d{2}$"),
    ("MD", r"^[A-Z\d]{20}$"),
    ("ME", r"^\d{18}$"),
    ("MK", r"^\d{3}[A-Z\d]{10}\d{2}$"),
    ("MR", r"^\d{23}$"),
    ("MT", r"^[A-Z]{4}\d{5}[A-Z\d]{18}$"),
    ("MU", r"^[A-Z]{4}\d{19}[A-Z]{3}$"),
    ("NL", r"^[A-Z]{4}\d{10}$"),
    ("NO", r"^\d{11}$"),
    ("PK", r"^[A-Z]{4}[A-Z\d]{16}$"),
    ("PL", r"^\d{24}$"),
    ("PS", r"^[A-Z]{4}[A-Z\d]{21}$"),
    ("PT", r"^\d{21}$"),
    ("QA", r"^[A-Z]{4}[A-Z\d]{21}$"),
    ("RO", r"^[A-z]{4}[A-Z\d]{16}$"),
    ("RS", r"^\d{18}$"),
    ("SA", r"^\d{2}[A-Z\d]{18}$"),
    ("SC", r"^[A-Z]{4}\d{20}[A-Z]{3}$"),
    ("SE", r"^\d{20}$"),
    ("SI", r"^\d{15}$"),
    ("SK", r"^\d{20}$"),
    ("SM", r"^[A-Z]{1}\d{10}[A-Z\d]{12}$"),
    ("ST", r"^\d{21}$"),
    ("SV", r"^[A-Z]{4}\d{20}$"),
    ("TL", r"^\d{19}$"),
    ("TN", r"^\d{20}$"),
    ("TR", r"^\d{6}[A-Z\d]{16}$"),
    ("UA", r"^\d{6}[A-Z\d]{19}$"),
    ("VA", r"^\d{18}$"),
    ("VG", r"^[A-Z]{4}\d{16}$"),
    ("XK", r"^\d{16}$"),
];

lazy_static! {
    pub static ref RE_COUNTRY_CODE: RegexSet =
        RegexSetBuilder::new(COUNTRY_FORMATS.iter().map(|&(re, _)| re))
            .build()
            .expect(
                "Could not compile regular expression for country codes. \
                 Please file an issue at https://github.com/ThomasdenH/iban_validate."
            );
}

lazy_static! {
    pub static ref RE_ADDRESS_REMAINDER: RegexSet =
        RegexSetBuilder::new(COUNTRY_FORMATS.iter().map(|&(_, re)| re))
            .size_limit(16_000_000)
            .build()
            .expect(
                "Could not compile regular expression for IBAN addresses. \
                 Please file an issue at https://github.com/ThomasdenH/iban_validate."
            );
}
