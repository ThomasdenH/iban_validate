//! This module tests the split utility functions provided by the [`Iban`] type.

use iban::{Iban, IbanLike};

#[test]
/// This test checks whether the different splits of an address are correct.
fn test_split() -> anyhow::Result<()> {
    let address: Iban = "AD1200012030200359100100".parse()?;
    assert_eq!(address.country_code(), "AD");
    assert_eq!(address.check_digits_str(), "12");
    assert_eq!(address.check_digits(), 12);
    assert_eq!(address.bban(), "00012030200359100100");

    let address: Iban = "TR330006100519786457841326".parse()?;
    assert_eq!(address.country_code(), "TR");
    assert_eq!(address.check_digits_str(), "33");
    assert_eq!(address.check_digits(), 33);
    assert_eq!(address.bban(), "0006100519786457841326");
    Ok(())
}
