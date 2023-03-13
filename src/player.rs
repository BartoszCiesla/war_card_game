use crate::card::Card;
use crate::draw::{Draw, PlayerId};

use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub(crate) struct Player {
    player_id: PlayerId,
    hand: VecDeque<Card>,
}

impl Player {
    pub(crate) fn new(player_id: u8) -> Self {
        Player {
            player_id,
            hand: VecDeque::new(),
        }
    }

    pub(crate) fn add_card(&mut self, card: Card) {
        self.hand.push_back(card)
    }

    pub(crate) fn get_card(&mut self) -> Option<Draw> {
        match self.hand.pop_front() {
            Some(card) => Some(Draw::new(self.player_id, card)),
            None => None,
        }
    }

    pub(crate) fn failed(&self) -> bool {
        self.hand.is_empty()
    }

    #[cfg(test)]
    pub(crate) fn size(&self) -> usize {
        self.hand.len()
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "P{}: ", self.player_id)?;
        self.hand.iter().fold(Ok(()), |result, card| {
            result.and_then(|_| write!(f, "{card} "))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::color::Color;
    use crate::rank::Rank;

    #[test]
    fn test_add_card() {
        let mut player = Player::new(1);

        player.add_card(Card::new(Rank::Ace, Color::Spades));
        player.add_card(Card::new(Rank::King, Color::Hearts));

        assert_eq!(2, player.size());
    }

    #[test]
    fn test_get_card() {
        let mut player = Player::new(2);

        player.add_card(Card::new(Rank::Two, Color::Diamonds));
        player.add_card(Card::new(Rank::Jack, Color::Hearts));

        assert_eq!(2, player.size());
        assert!(player.get_card().is_some());
        assert!(player.get_card().is_some());
        assert!(player.get_card().is_none());
    }

    #[test]
    fn test_failed() {
        let mut player = Player::new(3);

        player.add_card(Card::new(Rank::Seven, Color::Spades));
        player.add_card(Card::new(Rank::Queen, Color::Clubs));

        assert_eq!(2, player.size());
        assert!(!player.failed());
        assert!(player.get_card().is_some());
        assert!(!player.failed());
        assert!(player.get_card().is_some());
        assert!(player.failed());
    }

    #[test]
    fn test_display() {
        let mut player = Player::new(4);

        player.add_card(Card::new(Rank::Nine, Color::Diamonds));
        player.add_card(Card::new(Rank::King, Color::Spades));

        assert_eq!("P4: 🃉  🂮  ", player.to_string());
    }
}
