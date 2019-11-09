//! This module tests the parsing of various IBAN numbers

use crate::{Iban, ParseBaseIbanError, ParseIbanError};

#[test]
/// This test checks whether IBANs of invalid lengths are detected to be invalid.
fn test_length() {
    let invalid_lengths = ["DE4", "DE445001023460732493147896512575467"];

    for &i in invalid_lengths.into_iter() {
        assert_eq!(
            i.parse::<Iban>(),
            Err(ParseIbanError::InvalidBaseIban {
                source: ParseBaseIbanError::InvalidLength
            })
        );
    }
}

#[test]
/// This test checks whether IBANs containing invalid characters are detected to be invalid.
fn test_characters() {
    assert_eq!(
        "DE44@0010234607324931".parse::<Iban>(),
        Err(ParseIbanError::InvalidBaseIban {
            source: ParseBaseIbanError::InvalidCharacter('@'),
        })
    );
    assert_eq!(
        "$A0380000000648510167519".parse::<Iban>(),
        Err(ParseIbanError::InvalidBaseIban {
            source: ParseBaseIbanError::InvalidCharacter('$'),
        })
    );
    assert_eq!(
        "tr330006100519786457465326".parse::<Iban>(),
        Err(ParseIbanError::InvalidBaseIban {
            source: ParseBaseIbanError::InvalidCharacter('t'),
        })
    );
}

#[test]
/// This test checks whether IBANs having an invalid structure are detected to be invalid.
fn test_structure() {
    assert_eq!(
        "G416011012500000834112300695".parse::<Iban>(),
        Err(ParseIbanError::InvalidBaseIban {
            source: ParseBaseIbanError::InvalidStructure,
        })
    );
    assert_eq!(
        "CHI300762011623852957".parse::<Iban>(),
        Err(ParseIbanError::InvalidBaseIban {
            source: ParseBaseIbanError::InvalidStructure,
        })
    );
}

#[test]
/// This test checks whether IBANs having an invalid checksum are detected as such.
fn test_checksum() {
    let invalid_checksums = [
        "DE4450010234607324931",
        "GR16011012500000834112300695",
        "GB29NWBK60934331926819",
        "SA0380000000648510167519",
        "CH9300762011645852957",
        "TR330006100519786457465326",
    ];

    for &i in invalid_checksums.into_iter() {
        assert_eq!(
            i.parse::<Iban>(),
            Err(ParseIbanError::InvalidBaseIban {
                source: ParseBaseIbanError::InvalidChecksum
            })
        );
    }
}

#[test]
/// This test checks whether valid IBANs are marked valid.
fn test_valid_iban() {
    let valid_ibans = [
        "DE44500105175407324931",
        "GR1601101250000000012300695",
        "GB29NWBK60161331926819",
        "SA0380000000608010167519",
        "CH9300762011623852957",
        "TR330006100519786457841326",
    ];

    for &i in valid_ibans.into_iter() {
        assert!(i.parse::<Iban>().is_ok());
    }
}
