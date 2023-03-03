use crate::card::Card;

use std::fmt::Display;

#[derive(Debug)]
pub(crate) struct Player {
    pub player_id: u8,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(id: u8) -> Player {
        Player {
            player_id: id,
            hand: Vec::new(),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P{}: ", self.player_id)?;
        self.hand.iter().fold(Ok(()), |result, card| {
            result.and_then(|_| write!(f, "{card} "))
        })
    }
}
