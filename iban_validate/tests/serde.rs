#![cfg(feature = "serde")]
use iban::{BaseIban, Iban, ParseIbanError};
use serde_test::{assert_tokens, Token};

#[test]
fn base_iban_compact() -> Result<(), ParseIbanError> {
    use serde_test::Configure;
    let address: &str = "KW81CBKU0000000000001234560101";
    let i: BaseIban = address.parse()?;
    assert_tokens(&i.compact(), &[Token::Str(address)]);
    Ok(())
}

#[test]
fn base_iban_readable() -> Result<(), ParseIbanError> {
    use serde_test::Configure;
    let address: &str = "KW81 CBKU 0000 0000 0000 1234 5601 01";
    let i: BaseIban = address.parse()?;
    assert_tokens(&i.readable(), &[Token::Str(address)]);
    Ok(())
}

#[test]
fn iban_compact() -> Result<(), ParseIbanError> {
    use serde_test::Configure;
    let address: &str = "KW81CBKU0000000000001234560101";
    let i: Iban = address.parse()?;
    assert_tokens(&i.compact(), &[Token::Str(address)]);
    Ok(())
}

#[test]
fn iban_readable() -> Result<(), ParseIbanError> {
    use serde_test::Configure;
    let address: &str = "KW81 CBKU 0000 0000 0000 1234 5601 01";
    let i: Iban = address.parse()?;
    assert_tokens(&i.readable(), &[Token::Str(address)]);
    Ok(())
}
