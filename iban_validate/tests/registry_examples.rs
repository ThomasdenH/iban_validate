//! Use the registry examples to validate the code. As you can see, there are a lot of errors/exceptions, unfortunately.

use iban::{Iban, IbanLike, ParseIbanError};
mod registry_examples_generated;

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
        if matches!(*country_code, "NI" | "RU" | "ST") {
            // For these country codes, the provided IBAN is actually invalid.
            // There is nothing that we can test in this case. For posterity,
            // test if they actually still fail.
            assert!(
                iban_electronic.parse::<Iban>().is_err(),
                "expected invalid IBAN, maybe the registry was updated?"
            );
            continue;
        }

        let iban_1 = iban_electronic.parse::<Iban>().unwrap_or_else(|_| {
            panic!(
                "could not parse electronic format of country {}",
                country_code
            )
        });

        // For the countries that abide by the pretty print format, check if the parsed IBAN is identical.
        // We could remove the whitespace and parse again, but that's probably not worth it.
        if matches!(*country_code, "BI" | "LY" | "SV" | "VA") {
            // These countries do not follow the pretty print format.
            assert!(
                iban_print.parse::<Iban>().is_err(),
                "expected non-conforming pretty print format, maybe the registry was updated?"
            );
        } else {
            let iban_2 = iban_print.parse::<Iban>().unwrap_or_else(|_| {
                panic!("could not parse print format of country {}", country_code)
            });
            assert_eq!(
                iban_1, iban_2,
                "parsed ibans should be identical, regardless of format"
            );
        }

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
        if matches!(*country_code, "BA" | "MK" | "SE") {
            // For these countries, the examples do not match.
            assert_ne!(
                iban_1.bank_identifier(),
                bank_identifier.as_deref(),
                "expected non-matchin bank identifier, maybe the registry was updated?"
            );
            // Test the length instead.
            assert_eq!(
                iban_1.bank_identifier().unwrap().len(),
                bank_identifier.unwrap().len(),
                "bank identifier lengths do not match for country {}",
                country_code
            );
        } else {
            assert_eq!(
                iban_1.bank_identifier(),
                bank_identifier.as_deref(),
                "bank identifiers do not match for country {}",
                country_code
            );
        }

        // Validate the branch identifier.
        if *country_code == "JO" {
            // There is no example here.
            assert!(
                branch_identifier.is_none(),
                "expected missing branch identifier, maybe the registry was updated?"
            );
        } else if matches!(*country_code, "BA") {
            // For these countries, the examples do not match.
            assert_ne!(
                iban_1.branch_identifier(),
                branch_identifier.as_deref(),
                "expected non-matching branch identifier, maybe the registry was updated?"
            );
            // Test the length instead.
            assert_eq!(
                iban_1.branch_identifier().unwrap().len(),
                branch_identifier.unwrap().len(),
                "branch identifier lengths do not match for country {}",
                country_code
            );
        } else {
            // Test that the branch identifier matches.
            assert_eq!(
                iban_1.branch_identifier(),
                *branch_identifier,
                "branch identifiers do not match for country {}",
                country_code
            );
        }

        // Validate the BBAN.
        let bban: String = bban
            .chars()
            .filter(|c: &char| c.is_ascii_alphanumeric())
            .collect();
        if matches!(*country_code, "BA" | "BR" | "CR" | "FI" | "IL") {
            // For these countries, the BBAN example does not match the IBAN.
            assert_ne!(
                iban_1.bban(),
                bban,
                "expected non-matching BBAN, maybe the registry was updated?"
            );
        } else {
            assert_eq!(
                iban_1.bban(),
                bban,
                "the bban doesn't match for country {}",
                country_code
            );
        }
    }
    Ok(())
}
