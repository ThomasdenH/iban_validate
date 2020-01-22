//! This module contains the code for the validation of the countries in the Swift IBAN registry. It
//! checks if the country code is recognized and then tries to match the regular expression.

#[derive(Copy, Clone)]
pub(super) enum CharacterType {
    A,
    D,
    AOrD
}

impl CharacterType {
    fn match_iter(self, iter: &mut impl Iterator<Item = u8>) -> bool {
        use CharacterType::*;
        match (self, iter.next()) {
            (A, Some(a)) if a.is_ascii_uppercase() => true,
            (D, Some(a)) if (a as char).is_digit(10) => true,
            (AOrD, Some(a)) if a.is_ascii_uppercase() || (a as char).is_digit(10) => true,
            _ => false
        }
    }
}

#[derive(Copy, Clone)]
pub(super) struct N(pub(super) CharacterType, pub(super) usize);

impl N {
    fn match_iter(self, iter: &mut impl Iterator<Item = u8>) -> bool {
        (0..self.1).all(|_| self.0.match_iter(iter))
    }
}

pub(super) trait Matchable {
    fn match_str(self, s: &str) -> bool;
}

impl Matchable for &'_[N] {
    fn match_str(self, s: &str) -> bool {
        if s.len() != len(self) {
            return false;
        }
        let mut iter = s.as_bytes().iter().copied();
        for n in self {
            if !n.match_iter(&mut iter) {
                return false;
            }
        }
        true
    }
}

fn len(a: &[N]) -> usize {
    a.iter().map(|N(_, len)| len).sum()
}
