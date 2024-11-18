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
            let _ = iban.bank_identifier();
            let _ = iban.branch_identifier();

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

proptest! {
    #[test]
    #[cfg(feature = "arbitrary")]
    fn arbitrary_base_iban_generation(bytes in prop::collection::vec(prop::num::u8::ANY, 0..34)) {
        use arbitrary::{Unstructured, Arbitrary};
        let mut u = Unstructured::new(&bytes);
        let _iban = <BaseIban as Arbitrary>::arbitrary(&mut u).expect("bytes should be enough");
    }
}

proptest! {
    #[test]
    #[cfg(feature = "proptest")]
    fn proptest_base_iban_generation(iban in any::<BaseIban>()) {
        let _: BaseIban = iban.electronic_str().parse().unwrap();
        let _: BaseIban = iban.to_string().parse().unwrap();
    }
}

proptest! {
    #[test]
    #[cfg(feature = "arbitrary")]
    fn arbitrary_iban_generation(bytes in prop::collection::vec(prop::num::u8::ANY, 0..34)) {
        use arbitrary::{Unstructured, Arbitrary};
        let mut u = Unstructured::new(&bytes);
        let _iban = <Iban as Arbitrary>::arbitrary(&mut u).expect("bytes should be enough");
    }
}

proptest! {
    #[test]
    #[cfg(feature = "proptest")]
    fn proptest_iban_generation(iban in any::<Iban>()) {
        let _: Iban = iban.electronic_str().parse().unwrap();
        let _: Iban = iban.to_string().parse().unwrap();
    }
}
