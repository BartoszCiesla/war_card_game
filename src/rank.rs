use crate::unicode::ToUnicode;

use enum_iterator::Sequence;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Sequence)]
pub(crate) enum Rank {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_display() {
        assert_eq!("② ", Rank::Two.to_string());
        assert_eq!("③ ", Rank::Three.to_string());
        assert_eq!("④ ", Rank::Four.to_string());
        assert_eq!("⑤ ", Rank::Five.to_string());
        assert_eq!("⑥ ", Rank::Six.to_string());
        assert_eq!("⑦ ", Rank::Seven.to_string());
        assert_eq!("⑧ ", Rank::Eight.to_string());
        assert_eq!("⑨ ", Rank::Nine.to_string());
        assert_eq!("⑩ ", Rank::Ten.to_string());
        assert_eq!("Ⓙ ", Rank::Jack.to_string());
        assert_eq!("Ⓠ ", Rank::Queen.to_string());
        assert_eq!("Ⓚ ", Rank::King.to_string());
        assert_eq!("Ⓐ ", Rank::Ace.to_string());
    }

    #[test]
    fn test_to_unicode() {
        assert_eq!(2, Rank::Two.to_unicode());
        assert_eq!(3, Rank::Three.to_unicode());
        assert_eq!(4, Rank::Four.to_unicode());
        assert_eq!(5, Rank::Five.to_unicode());
        assert_eq!(6, Rank::Six.to_unicode());
        assert_eq!(7, Rank::Seven.to_unicode());
        assert_eq!(8, Rank::Eight.to_unicode());
        assert_eq!(9, Rank::Nine.to_unicode());
        assert_eq!(10, Rank::Ten.to_unicode());
        assert_eq!(11, Rank::Jack.to_unicode());
        assert_eq!(13, Rank::Queen.to_unicode());
        assert_eq!(14, Rank::King.to_unicode());
        assert_eq!(1, Rank::Ace.to_unicode());
    }
}
