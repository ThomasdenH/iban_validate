//! This module tests the split utility functions provided by the [`Iban`] type.

use Iban;

#[test]
/// This test checks whether the different splits of an address are correct.
fn test_split() {
    let address: Iban = "AD1200012030200359100100".parse().unwrap();
    assert_eq!(address.get_country_code(), "AD");
    assert_eq!(address.get_check_digits(), "12");
    assert_eq!(address.get_bban(), "00012030200359100100");


    let address: Iban = "TR330006100519786457841326".parse().unwrap();
    assert_eq!(address.get_country_code(), "TR");
    assert_eq!(address.get_check_digits(), "33");
    assert_eq!(address.get_bban(), "0006100519786457841326");
}
