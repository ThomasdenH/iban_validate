//! Use the registry examples to validate the code. As you can see, there are a lot of errors/exceptions, unfortunately.

use iban::{Iban, IbanLike, ParseIbanError};

pub mod registry_examples_generated;
use registry_examples_generated::{RegistryExample, EXAMPLES};

#[test]
fn test_registry_examples() -> Result<(), ParseIbanError> {
    for RegistryExample {
        country_code,
        bank_identifier,
        branch_identifier,
        bban,
        iban_electronic,
        iban_print,
    } in EXAMPLES
    {
        let iban_1 = iban_electronic.parse::<Iban>().unwrap_or_else(|_| {
            panic!(
                "could not parse electronic format of country {}",
                country_code
            )
        });

        // For the countries that abide by the pretty print format, check if the parsed IBAN is identical.
        // We could remove the whitespace and parse again, but that's probably not worth it.
        let iban_2 = iban_print
            .parse::<Iban>()
            .unwrap_or_else(|_| panic!("could not parse print format of country {}", country_code));
        assert_eq!(
            iban_1, iban_2,
            "parsed ibans should be identical, regardless of format"
        );

        // Validate the country code.
        assert_eq!(
            iban_1.country_code(),
            *country_code,
            "country codes do not match for country {}",
            country_code
        );

        // Validate the bank identifier.
        let bank_identifier: Option<String> =
            bank_identifier.map(|c| c.chars().filter(|c| c.is_ascii_alphanumeric()).collect());

        assert_eq!(
            iban_1.bank_identifier(),
            bank_identifier.as_deref(),
            "bank identifiers do not match for country {}",
            country_code
        );

        // Test that the branch identifier matches.
        assert_eq!(
            iban_1.branch_identifier(),
            *branch_identifier,
            "branch identifiers do not match for country {}",
            country_code
        );

        // Validate the BBAN.
        let bban: String = bban
            .chars()
            .filter(|c: &char| c.is_ascii_alphanumeric())
            .collect();

        assert_eq!(
            iban_1.bban(),
            bban,
            "the bban doesn't match for country {}",
            country_code
        );
    }
    Ok(())
}
