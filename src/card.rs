use crate::color::Color;
use crate::rank::Rank;
use crate::unicode::ToUnicode;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) struct Card {
    color: Color,
    rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let number = self.color.to_unicode() + self.rank.to_unicode();
        write!(f, "{} ", std::char::from_u32(number).unwrap_or('�'))
    }
}

impl Card {
    pub fn new(rank: Rank, color: Color) -> Card {
        Card { rank, color }
    }
}
