use Iban;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10_000, .. ProptestConfig::default()
      })]
    #[test]
    fn parse_iban_format_electronic(country_code in "[A-Z]{2}",
            check_digits in 0u8..=99u8,
            bban in "[A-Z0-9]{1,30}") {

        println!("{}{:02}{}", country_code, check_digits, bban);

        let iban_string = format!("{}{:02}{}", country_code, check_digits, bban);

        match iban_string.parse::<Iban>() {
            Ok(iban) => {
                // Validate country
                iban.validate_bban();

                // Format
                assert_eq!(iban.format_electronic(), iban_string);

                // Split
                assert_eq!(iban.get_country_code(), country_code);
                assert_eq!(iban.get_check_digits(), check_digits);
                assert_eq!(iban.get_bban(), bban);

                // Convert to string and parse again
                let print_string = iban.format_print();
                assert_eq!(print_string.parse::<Iban>().unwrap(), iban);
            },
            Err(_) => {
                // Invalid checksum
            }
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10_000, .. ProptestConfig::default()
      })]
    #[test]
    fn doesnt_crash_random_input(s in "\\PC*") {
        let _ = s.parse::<Iban>();
    }
}
