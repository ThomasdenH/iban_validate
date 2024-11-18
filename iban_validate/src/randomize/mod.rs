//! This module contains methods for generating random IBANs.
//!
//! We have two implementations, one for `arbitrary` and one for `rand`. The
//! implementation of [`proptest::Arbitrary`] just uses the one from
//! `arbitrary` internally.

#[cfg(any(feature = "arbitrary", feature = "rand"))]
use core::ops::RangeInclusive;

#[cfg(any(feature = "arbitrary", feature = "rand"))]
use crate::{base_iban::MAX_IBAN_LEN, countries::CharacterType};

/// This trait is shared beween [`arbitrary`], [`proptest`] and [`rand`].
#[cfg(any(feature = "arbitrary", feature = "rand", feature = "proptest"))]
pub(crate) trait RandomGeneration {
    type Error;
    fn gen_u8_range(&mut self, range: RangeInclusive<u8>) -> Result<u8, Self::Error>;
    fn gen_u16_range(&mut self, range: RangeInclusive<u16>) -> Result<u16, Self::Error>;

    #[inline]
    /// Generate one digit of an IBAN.
    fn generate_digit(&mut self, character_type: CharacterType) -> Result<u8, Self::Error> {
        match character_type {
            CharacterType::A => self.gen_u8_range(b'A'..=b'Z'),
            CharacterType::N => self.gen_u8_range(b'0'..=b'9'),
            CharacterType::C => self.gen_u8_range(0u8..=(26 + 10 - 1)).map(|int| match int {
                0..10 => int + b'0',
                10..36 => int + b'A' - 10,
                _ => unreachable!(),
            }),
        }
    }

    #[inline]
    fn generate_iban_len(&mut self) -> Result<usize, Self::Error> {
        self.gen_u8_range(1..=u8::try_from(MAX_IBAN_LEN).expect("this should fit") - 4)
            .map(usize::from)
    }
}

#[cfg(feature = "rand")]
pub(crate) struct RandRandomGeneration<'a, R>(pub(crate) &'a mut R)
where
    R: rand::Rng + ?Sized;
#[cfg(feature = "rand")]
impl<'a, R> RandomGeneration for RandRandomGeneration<'a, R>
where
    R: rand::Rng + ?Sized,
{
    type Error = ();
    #[inline]
    fn gen_u8_range(&mut self, range: RangeInclusive<u8>) -> Result<u8, Self::Error> {
        Ok(self.0.gen_range(range))
    }
    #[inline]
    fn gen_u16_range(&mut self, range: RangeInclusive<u16>) -> Result<u16, Self::Error> {
        Ok(self.0.gen_range(range))
    }
}

#[cfg(feature = "arbitrary")]
pub(crate) struct ArbitraryRandomGeneration<'a, 'b>(pub(crate) &'b mut arbitrary::Unstructured<'a>);
#[cfg(feature = "arbitrary")]
impl<'a, 'b> RandomGeneration for ArbitraryRandomGeneration<'a, 'b> {
    type Error = arbitrary::Error;
    #[inline]
    fn gen_u8_range(&mut self, range: RangeInclusive<u8>) -> Result<u8, Self::Error> {
        self.0.int_in_range(range)
    }
    #[inline]
    fn gen_u16_range(&mut self, range: RangeInclusive<u16>) -> Result<u16, Self::Error> {
        self.0.int_in_range(range)
    }
}
