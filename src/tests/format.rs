//! This module tests the formatting of IBANs

use super::TestResult;
use crate::{Iban, IbanLike};

/// This test checks the electronic formatting method.
#[test]
fn electronic() -> TestResult {
    // An IBAN without spaces
    assert_eq!(
        "BE68539007547034".parse::<Iban>()?.electronic_str(),
        "BE68539007547034"
    );

    // An IBAN in the pretty print format
    assert_eq!(
        "BE68 5390 0754 7034".parse::<Iban>()?.electronic_str(),
        "BE68539007547034"
    );
    Ok(())
}

/// This test checks the print formatting method.
#[test]
fn print() -> TestResult {
    // An IBAN without spaces
    assert_eq!(
        "KW81CBKU0000000000001234560101"
            .parse::<Iban>()?
            .to_string(),
        "KW81 CBKU 0000 0000 0000 1234 5601 01"
    );

    // An IBAN in the pretty print format
    assert_eq!(
        "PL61 1090 1014 0000 0712 1981 2874"
            .parse::<Iban>()?
            .to_string(),
        "PL61 1090 1014 0000 0712 1981 2874"
    );
    Ok(())
}
