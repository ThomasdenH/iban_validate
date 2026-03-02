#![cfg(feature = "zeroize")]

use iban::{BaseIban, IbanLike};
use zeroize::Zeroize;

#[test]
fn zeroize() {
    let mut base_iban = "DE44500105175407324931"
        .parse::<BaseIban>()
        .expect("valid IBAN");

    assert_eq!(
        base_iban.electronic_str(),
        String::from("DE44500105175407324931")
    );

    base_iban.zeroize();

    assert_eq!(
        base_iban.electronic_str(),
        String::from("\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
    );
}
