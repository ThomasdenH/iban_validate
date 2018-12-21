//! This module tests the split utility functions provided by the [`Iban`] type.

use crate::Iban;
use expectest::expect;
use expectest::prelude::*;

#[test]
/// This test checks whether the different splits of an address are correct.
fn test_split() {
    let address: Iban = "AD1200012030200359100100".parse().unwrap();
    expect!(address.get_country_code()).to(be_equal_to("AD"));
    expect!(address.get_check_digits()).to(be_equal_to(12));
    expect!(address.get_bban()).to(be_equal_to("00012030200359100100"));

    let address: Iban = "TR330006100519786457841326".parse().unwrap();
    expect!(address.get_country_code()).to(be_equal_to("TR"));
    expect!(address.get_check_digits()).to(be_equal_to(33));
    expect!(address.get_bban()).to(be_equal_to("0006100519786457841326"));
}
