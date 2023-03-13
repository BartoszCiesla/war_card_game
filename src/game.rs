use crate::card::Card;
use crate::color::Color;
use crate::player::Player;
use crate::rank::Rank;

use enum_iterator::all;
use itertools::iproduct;
use std::fmt::Display;

#[derive(Debug)]
pub(crate) struct Game {
    pub deck: Vec<Card>,
    pub hands: Vec<Player>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.deck.is_empty() {
            write!(f, "Deck: ")?;
            self.deck.iter().fold(Ok(()), |result, card| {
                result.and_then(|_| write!(f, "{card} "))
            })
        } else {
            self.hands.iter().fold(Ok(()), |result, hand| {
                result.and_then(|_| writeln!(f, "{hand}"))
            })
        }
    }
}

impl Game {
    pub fn new() -> Game {
        let ranks = all::<Rank>().collect::<Vec<_>>();
        let colors = all::<Color>().collect::<Vec<_>>();
        Game {
            deck: iproduct!(ranks.into_iter(), colors.into_iter())
                .map(|(r, c)| Card::new(r, c))
                .collect::<Vec<_>>(),
            hands: vec![
                Player::new(1),
                Player::new(2),
                Player::new(3),
                Player::new(4),
            ],
        }
    }

    pub fn deal(&mut self) {
        let quantity = self.deck.len() / self.hands.len() + 1;
        for _ in 0..quantity {
            for hand in self.hands.iter_mut() {
                let card = self.deck.pop();
                match card {
                    Some(item) => hand.add_card(item),
                    None => break,
                }
            }
        }
    }
}
