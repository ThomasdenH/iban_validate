use iban::{BaseIban, Iban, IbanLike};
use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_iban_format_electronic(country_code in "[A-Z]{2}",
            check_digits in 2_u8..=99_u8,
            bban in "[A-Z0-9]{1,30}") {

        let iban_string = format!("{}{:02}{}", country_code, check_digits, bban);

        if let Ok(iban) = iban_string.parse::<Iban>() {
            // Format
            assert_eq!(iban.electronic_str(), iban_string);

            // Split
            assert_eq!(iban.country_code(), country_code);
            assert_eq!(iban.check_digits(), check_digits);
            assert_eq!(iban.bban(), bban);
            iban.bank_identifier();
            iban.branch_identifier();

            // Convert to string and parse again
            let print_string = iban.to_string();
            assert_eq!(print_string.parse::<Iban>().unwrap(), iban);
        }
    }
}

proptest! {
    #[test]
    fn parse_base_iban_format_electronic(country_code in "[A-Z]{2}",
            check_digits in 2_u8..=99_u8,
            bban in "[A-Z0-9]{1,30}") {

        let iban_string = format!("{}{:02}{}", country_code, check_digits, bban);

        if let Ok(iban) = iban_string.parse::<BaseIban>() {
                // Format
                assert_eq!(iban.electronic_str(), iban_string);

                // Split
                assert_eq!(iban.country_code(), country_code);
                assert_eq!(iban.check_digits(), check_digits);
                assert_eq!(iban.bban_unchecked(), bban);

                // Convert to string and parse again
                let print_string = iban.to_string();
                assert_eq!(print_string.parse::<BaseIban>().unwrap(), iban);
        }
    }
}

proptest! {
    #[test]
    fn doesnt_crash_random_input(s in "\\PC*") {
        let _ = s.parse::<Iban>();
    }
}
