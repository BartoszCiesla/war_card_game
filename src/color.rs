use crate::unicode::ToUnicode;

use enum_iterator::Sequence;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Clone, Copy, Sequence, Eq, PartialEq)]
pub(crate) enum Color {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Clubs => write!(f, "\u{2660} "),
            Self::Diamonds => write!(f, "\u{2666} "),
            Self::Hearts => write!(f, "\u{2665} "),
            Self::Spades => write!(f, "\u{2663} "),
        }
    }
}

impl ToUnicode for Color {
    fn to_unicode(&self) -> u32 {
        match self {
            Self::Clubs => 0x1F0D0,
            Self::Diamonds => 0x1F0C0,
            Self::Hearts => 0x1F0B0,
            Self::Spades => 0x1F0A0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("♠ ", Color::Clubs.to_string());
        assert_eq!("♦ ", Color::Diamonds.to_string());
        assert_eq!("♥ ", Color::Hearts.to_string());
        assert_eq!("♣ ", Color::Spades.to_string());
    }

    #[test]
    fn test_to_unicode() {
        assert_eq!(127184, Color::Clubs.to_unicode());
        assert_eq!(127168, Color::Diamonds.to_unicode());
        assert_eq!(127152, Color::Hearts.to_unicode());
        assert_eq!(127136, Color::Spades.to_unicode());
    }
}
