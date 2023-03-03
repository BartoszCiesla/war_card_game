use crate::unicode::ToUnicode;

use enum_iterator::Sequence;
use std::fmt::Display;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Sequence)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Two => write!(f, "\u{2461} "),
            Self::Three => write!(f, "\u{2462} "),
            Self::Four => write!(f, "\u{2463} "),
            Self::Five => write!(f, "\u{2464} "),
            Self::Six => write!(f, "\u{2465} "),
            Self::Seven => write!(f, "\u{2466} "),
            Self::Eight => write!(f, "\u{2467} "),
            Self::Nine => write!(f, "\u{2468} "),
            Self::Ten => write!(f, "\u{2469} "),
            Self::Jack => write!(f, "\u{24BF} "),
            Self::Queen => write!(f, "\u{24C6} "),
            Self::King => write!(f, "\u{24C0} "),
            Self::Ace => write!(f, "\u{24B6} "),
        }
    }
}

impl ToUnicode for Rank {
    fn to_unicode(&self) -> u32 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 0xA,
            Self::Jack => 0xB,
            Self::Queen => 0xD,
            Self::King => 0xE,
            Self::Ace => 1,
        }
    }
}
