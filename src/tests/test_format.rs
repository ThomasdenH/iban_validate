//! This module tests the formatting of IBANs

use crate::Iban;
use expectest::expect;
use expectest::prelude::*;

/// This test checks the electronic formatting method.
#[test]
fn test_format_electronic() {
    // An IBAN without spaces
    expect!("KW81CBKU0000000000001234560101"
        .parse::<Iban>()
        .unwrap()
        .format_electronic())
    .to(be_equal_to("KW81CBKU0000000000001234560101"));

    // An IBAN in the pretty print format
    expect!("PL61 1090 1014 0000 0712 1981 2874"
        .parse::<Iban>()
        .unwrap()
        .format_electronic())
    .to(be_equal_to("PL61109010140000071219812874"));

    // An IBAN with random spaces
    expect!("TL 380 080 0123 4567 8910 157"
        .parse::<Iban>()
        .unwrap()
        .format_electronic())
    .to(be_equal_to("TL380080012345678910157"));
}

/// This test checks the print formatting method.
#[test]
fn test_format_print() {
    // An IBAN without spaces
    expect!("KW81CBKU0000000000001234560101"
        .parse::<Iban>()
        .unwrap()
        .format_print())
    .to(be_equal_to("KW81 CBKU 0000 0000 0000 1234 5601 01"));

    // An IBAN in the pretty print format
    expect!("PL61 1090 1014 0000 0712 1981 2874"
        .parse::<Iban>()
        .unwrap()
        .format_print())
    .to(be_equal_to("PL61 1090 1014 0000 0712 1981 2874"));

    // An IBAN with random spaces
    expect!("TL 380 080 0123 4567 8910 157"
        .parse::<Iban>()
        .unwrap()
        .format_print())
    .to(be_equal_to("TL38 0080 0123 4567 8910 157"));
}

/// This test checks the implementation of the Display trait
#[test]
fn test_format() {
    let iban1 = "KW81CBKU0000000000001234560101".parse::<Iban>().unwrap();
    expect!(format!("{}", iban1)).to(be_equal_to("KW81 CBKU 0000 0000 0000 1234 5601 01"));
    let iban1 = "PL61 1090 1014 0000 0712 1981 2874"
        .parse::<Iban>()
        .unwrap();
    expect!(format!("{}", iban1)).to(be_equal_to("PL61 1090 1014 0000 0712 1981 2874"));
    let iban1 = "TL 380 080 0123 4567 8910 157".parse::<Iban>().unwrap();
    expect!(format!("{}", iban1)).to(be_equal_to("TL38 0080 0123 4567 8910 157"));
}
