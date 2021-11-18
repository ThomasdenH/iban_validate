//! This module contains tests for the BBAN format

use iban::{BaseIban, Iban, ParseIbanError};

#[test]
/// This test checks whether ibans with a valid country format are recognized as such.
fn test_valid_countries() -> Result<(), ParseIbanError> {
    let valid_iban_countries = [
        "AD1200012030200359100100",
        "AE070331234567890123456",
        "AL47212110090000000235698741",
        "AT611904300234573201",
        "AZ21NABZ00000000137010001944",
        "BA391290079401028494",
        "BE68539007547034",
        "BG80BNBG96611020345678",
        "BH67BMAG00001299123456",
        "BR1800360305000010009795493C1",
        "BY13NBRB3600900000002Z00AB00",
        "CH9300762011623852957",
        "CR05015202001026284066",
        "CY17002001280000001200527600",
        "CZ6508000000192000145399",
        "DE89370400440532013000",
        "DK5000400440116243",
        "DO28BAGR00000001212453611324",
        "EE382200221020145685",
        "ES9121000418450200051332",
        "FI2112345600000785",
        "FO6264600001631634",
        "FR1420041010050500013M02606",
        "GB29NWBK60161331926819",
        "GE29NB0000000101904917",
        "GI75NWBK000000007099453",
        "GL8964710001000206",
        "GR1601101250000000012300695",
        "GT82TRAJ01020000001210029690",
        "HR1210010051863000160",
        "HU42117730161111101800000000",
        "IE29AIBK93115212345678",
        "IL620108000000099999999",
        "IQ98NBIQ850123456789012",
        "IS140159260076545510730339",
        "IT60X0542811101000000123456",
        "JO94CBJO0010000000000131000302",
        "KW81CBKU0000000000001234560101",
        "KZ86125KZT5004100100",
        "LB62099900000001001901229114",
        "LC55HEMM000100010012001200023015",
        "LI21088100002324013AA",
        "LT121000011101001000",
        "LU280019400644750000",
        "LV80BANK0000435195001",
        "LY83002048000020100120361",
        "MC5811222000010123456789030",
        "MD24AG000225100013104168",
        "ME25505000012345678951",
        "MK07250120000058984",
        "MR1300020001010000123456753",
        "MT84MALT011000012345MTLCAST001S",
        "MU17BOMM0101101030300200000MUR",
        "NL91ABNA0417164300",
        "NO9386011117947",
        "PK36SCBL0000001123456702",
        "PL61109010140000071219812874",
        "PS92PALS000000000400123456702",
        "PT50000201231234567890154",
        "QA58DOHB00001234567890ABCDEFG",
        "RO49AAAA1B31007593840000",
        "RS35260005601001611379",
        "SA0380000000608010167519",
        "SC18SSCB11010000000000001497USD",
        "SE4550000000058398257466",
        "SI56263300012039086",
        "SK3112000000198742637541",
        "SM86U0322509800000000270100",
        "ST68000100010051845310112",
        "SV62CENR00000000000000700025",
        "TL380080012345678910157",
        "TN5910006035183598478831",
        "TR330006100519786457841326",
        "UA213223130000026007233566001",
        "VG96VPVG0000012345678901",
        "XK051212012345678906",
        "VA59001123000012345678",
    ];

    for &i in valid_iban_countries.iter() {
        i.parse::<Iban>()?;
    }
    Ok(())
}

#[test]
/// This test checks whether invalid country formats are recognized as such.
fn test_invalid_country_format() -> Result<(), ParseIbanError> {
    let valid_iban_counties = [
        "AD54BD012030200359100100",
        "AE32ABCD234567890123456",
        "AL84212110090000AB023569874",
        "AT24190430234533203672",
        "AZ75N00Z000000000137010001944",
        "BA6312900794010284AC",
        "BE095390075470",
        "BG83BN96611020345678",
        "BH93BG00001299123456",
        "BR15003605000010009795493C1",
        "BY56NBRB36009000002Z00AB00",
    ];

    for &i in valid_iban_counties.iter() {
        let base_iban = i.parse::<BaseIban>()?;
        assert_eq!(
            i.parse::<Iban>(),
            Err(ParseIbanError::InvalidBban(base_iban))
        );
    }
    Ok(())
}

#[test]
/// This test checks whether an iban with an unknown country is recognized as such.
fn test_unknown_country() -> Result<(), ParseIbanError> {
    let iban_unknown_string = "ZZ07273912631298461";
    let base_iban = iban_unknown_string.parse::<BaseIban>()?;
    assert_eq!(
        iban_unknown_string.parse::<Iban>(),
        Err(ParseIbanError::UnknownCountry(base_iban))
    );
    Ok(())
}
