#[cfg(feature = "arbitrary")]
mod arb {
    use arbitrary::{Arbitrary, Unstructured};
    use iban::{BaseIban, Iban, IbanLike};
    use std::error::Error;

    #[test]
    fn test_base_iban_arbitrary() -> Result<(), Box<dyn Error>> {
        let bytes = &[46, 34, 28, 11, 1, 9];
        let mut u = Unstructured::new(bytes);
        assert_eq!(
            BaseIban::arbitrary(&mut u)?,
            "UI94B1900000000000000000000000000".parse()?
        );

        let bytes = &[1, 2, 3, 4, 5];
        let mut u = Unstructured::new(bytes);
        assert_eq!(BaseIban::arbitrary(&mut u)?, "BC044500".parse()?);

        Ok(())
    }

    #[test]
    fn test_iban_arbitrary() -> Result<(), Box<dyn Error>> {
        let bytes = &[46, 34, 28, 11, 1, 9];
        let mut u = Unstructured::new(bytes);
        assert_eq!(Iban::arbitrary(&mut u)?, "LT414811900000000000".parse()?);

        let bytes = &[1, 2, 3, 4, 5];
        let mut u = Unstructured::new(bytes);
        assert_eq!(Iban::arbitrary(&mut u)?, "AE692345000000000000000".parse()?);

        Ok(())
    }

    #[test]
    fn test_longest_iban() -> Result<(), Box<dyn Error>> {
        let bytes = &[
            255, 255, 29, 255, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1,
        ];
        let original_length = bytes.len();
        let mut u = Unstructured::new(bytes);
        let iban = BaseIban::arbitrary(&mut u)?;

        // 34 is the longest length.
        assert_eq!(iban.electronic_str().len(), 34);
        assert_eq!(iban, "VV82311111111111111111111111111111".parse()?);

        // We should not have used more than 34 bytes
        assert!(original_length - u.len() < 34);

        Ok(())
    }
}

#[cfg(feature = "rand")]
#[test]
/// Generate a random value.
fn test_random_base_iban_generation() {
    use iban::BaseIban;
    use rand::rngs::SmallRng;
    use rand::Rng;
    use rand::SeedableRng;

    let mut random = SmallRng::from_seed([0; 32]);
    // Generate IBANs
    for _ in 0..100 {
        let _: BaseIban = random.gen();
    }
}

#[cfg(feature = "rand")]
#[test]
/// Generate a random value.
fn test_random_iban_generation() {
    use iban::Iban;
    use rand::rngs::SmallRng;
    use rand::Rng;
    use rand::SeedableRng;

    let mut random = SmallRng::from_seed([0; 32]);
    // Generate IBANs
    for _ in 0..100 {
        let _: Iban = random.gen();
    }
}
