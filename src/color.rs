use crate::unicode::ToUnicode;

use enum_iterator::Sequence;
use std::fmt::Display;

#[derive(Debug, Clone, Sequence)]
pub(crate) enum Color {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
