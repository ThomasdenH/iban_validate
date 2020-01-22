#[derive(Copy, Clone)]
pub(super) enum CharacterType {
    C,
    N,
    A,
}

impl CharacterType {
    fn matches(self, c: u8) -> bool {
        use CharacterType::*;
        match self {
            A => c.is_ascii_uppercase(),
            N => (c as char).is_digit(10),
            C => c.is_ascii_uppercase() || (c as char).is_digit(10),
        }
    }
}

pub(super) trait Matchable {
    fn match_str(self, s: &str) -> bool;
}

impl Matchable for &'_ [(usize, CharacterType)] {
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
