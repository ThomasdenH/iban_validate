use iban::Iban;
use iban::IbanValidationResult;

#[test]
fn test_length() {
    let a = Iban::new("DE4");
    assert_eq!(a.validate(), IbanValidationResult::InvalidLength { l: 3 });

    let b = Iban::new("DE445001023460732493147896512575467");
    assert_eq!(b.validate(), IbanValidationResult::InvalidLength { l: 35 });
}

#[test]
fn test_characters() {
    let a = Iban::new("DE44@0010234607324931");
    assert_eq!(a.validate(), IbanValidationResult::InvalidCharacter { i: 4, c: '@' });

    let b = Iban::new("$A0380000000648510167519");
    assert_eq!(b.validate(), IbanValidationResult::InvalidCharacter { i: 0, c: '$' });

    let c = Iban::new("tr330006100519786457465326");
    assert_eq!(c.validate(), IbanValidationResult::InvalidCharacter { i: 0, c: 't' });

    let d = Iban::new("G416011012500000834112300695");
    assert_eq!(d.validate(), IbanValidationResult::InvalidCharacter { i: 1, c: '4' });

    let e = Iban::new("CHI300762011623852957");
    assert_eq!(e.validate(), IbanValidationResult::InvalidCharacter { i: 2, c: 'I' });
}

#[test]
fn test_checksum() {
    let a = Iban::new("DE4450010234607324931");
    assert_eq!(a.validate(), IbanValidationResult::InvalidChecksum { s: 69 });

    let b = Iban::new("GR16011012500000834112300695");
    assert_eq!(b.validate(), IbanValidationResult::InvalidChecksum { s: 5 });

    let c = Iban::new("GB29NWBK60934331926819");
    assert_eq!(c.validate(), IbanValidationResult::InvalidChecksum { s: 60 });

    let d = Iban::new("SA0380000000648510167519");
    assert_eq!(d.validate(), IbanValidationResult::InvalidChecksum { s: 29 });

    let e = Iban::new("CH9300762011645852957");
    assert_eq!(e.validate(), IbanValidationResult::InvalidChecksum { s: 34 });

    let f = Iban::new("TR330006100519786457465326");
    assert_eq!(f.validate(), IbanValidationResult::InvalidChecksum { s: 21 });
}

#[test]
fn test_valid_iban() {
    let a = Iban::new("DE44500105175407324931");
    assert_eq!(a.validate(), IbanValidationResult::Valid);

    let b = Iban::new("GR1601101250000000012300695");
    assert_eq!(b.validate(), IbanValidationResult::Valid);

    let c = Iban::new("GB29NWBK60161331926819");
    assert_eq!(c.validate(), IbanValidationResult::Valid);

    let d = Iban::new("SA0380000000608010167519");
    assert_eq!(d.validate(), IbanValidationResult::Valid);

    let e = Iban::new("CH9300762011623852957");
    assert_eq!(e.validate(), IbanValidationResult::Valid);

    let f = Iban::new("TR330006100519786457841326");
    assert_eq!(f.validate(), IbanValidationResult::Valid);
}