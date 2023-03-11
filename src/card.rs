use crate::color::Color;
use crate::rank::Rank;
use crate::unicode::ToUnicode;

use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Card {
    color: Color,
    rank: Rank,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank.eq(&other.rank)
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let number = self.color.to_unicode() + self.rank.to_unicode();
        write!(f, "{} ", char::from_u32(number).unwrap_or('\u{1F0A0}'))
    }
}

impl Card {
    pub fn new(rank: Rank, color: Color) -> Self {
        Card { rank, color }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_equal() {
        assert_eq!(
            Card::new(Rank::Two, Color::Clubs),
            Card::new(Rank::Two, Color::Diamonds)
        );
        assert_ne!(
            Card::new(Rank::Two, Color::Clubs),
            Card::new(Rank::Ace, Color::Diamonds)
        );
    }

    #[test]
    fn test_ord() {
        let mut input = vec![
            Card::new(Rank::King, Color::Hearts),
            Card::new(Rank::Two, Color::Clubs),
            Card::new(Rank::Ten, Color::Spades),
            Card::new(Rank::Two, Color::Diamonds),
        ];
        let sorted = vec![
            Card::new(Rank::Two, Color::Clubs),
            Card::new(Rank::Two, Color::Diamonds),
            Card::new(Rank::Ten, Color::Spades),
            Card::new(Rank::King, Color::Hearts),
        ];

        input.sort();

        assert_eq!(input, sorted);
    }

    #[test]
    fn test_display() {
        assert_eq!("🂳 ", Card::new(Rank::Three, Color::Hearts).to_string());
        assert_eq!("🂡 ", Card::new(Rank::Ace, Color::Spades).to_string());
    }
}
