//! A module for parsing the BBAN structures from a definition. The format is
//! very simple and can be optimized well by the compiler.

/// A `CharacterType` can match a single character. This corresponds to the
/// categories in the Swift registry for the most part, except that it doesn't
/// allow lowercase characters for `c`. However, when parsing we have
/// normalized the case anyway.
#[derive(Copy, Clone)]
pub(super) enum CharacterType {
    C,
    N,
    A,
}

impl CharacterType {
    fn matches(self, c: u8) -> bool {
        use CharacterType::{A, C, N};
        const MASK_CAPITAL: u8 = 0b0100_0000;
        const MASK_DIGIT: u8 = 0b0010_0000;
        debug_assert!(c.is_ascii_uppercase() || c.is_ascii_digit());
        match self {
            A => (c & MASK_DIGIT) == 0,
            N => (c & MASK_CAPITAL) == 0,
            C => true,
        }
    }
}

pub(super) trait Matchable {
    fn match_str(self, s: &str) -> bool;
}

impl Matchable for &'_ [(usize, CharacterType)] {
    /// Check if the string matches the format. The format is a list of counts
    /// followed by their character type. For example, [(3, A) (2, N)] would
    /// mean three letters followed by two numbers. The string should also have
    /// the correct length.
    fn match_str(self, s: &str) -> bool {
        s.len() == len(self)
            && self
                .iter()
                .flat_map(|(count, character_type)| (0..*count).map(move |_| character_type))
                .zip(s.as_bytes())
                .all(|(character_type, c)| character_type.matches(*c))
    }
}

fn len(a: &[(usize, CharacterType)]) -> usize {
    a.iter().map(|(count, _)| count).sum()
}
