use core::error::Error;

use iban::{BaseIban, Iban};

/// [`Iban`] implements [`AsRef`] to [`BaseIban`]. Test it here and see if it
/// displays the same.
#[test]
fn test_as_ref() -> Result<(), Box<dyn Error>> {
    let iban: Iban = "KW81CBKU0000000000001234560101".parse()?;

    fn pretty_format(base_iban: impl AsRef<BaseIban>) -> String {
        let base_iban: &BaseIban = base_iban.as_ref();
        base_iban.to_string()
    }

    let s = pretty_format(iban);
    assert_eq!(s.as_str(), "KW81 CBKU 0000 0000 0000 1234 5601 01");
    assert_eq!(iban.to_string(), s);

    Ok(())
}
