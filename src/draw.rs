use crate::card::Card;

use std::cmp::Ordering;

pub(crate) type PlayerId = u8;

#[derive(Debug)]
pub(crate) struct Draw {
    pub(crate) player_id: PlayerId,
    pub(crate) card: Card,
}

impl Draw {
    pub(crate) fn new(player_id: PlayerId, card: Card) -> Self {
        Draw { player_id, card }
    }

    pub(crate) fn cmp_by_card(&self, other: &Self) -> Ordering {
        self.card.cmp(&other.card)
    }
}

impl PartialEq for Draw {
    fn eq(&self, other: &Self) -> bool {
        self.player_id.eq(&other.player_id) && self.card.eq(&other.card)
    }
}

impl Eq for Draw {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::color::Color;
    use crate::rank::Rank;

    #[test]
    fn test_cmp_by_card() {
        let draw_one = Draw::new(1, Card::new(Rank::Ace, Color::Spades));
        let draw_two = Draw::new(2, Card::new(Rank::Ten, Color::Hearts));
        let draw_three = Draw::new(3, Card::new(Rank::Ace, Color::Diamonds));

        assert_eq!(Ordering::Greater, draw_one.cmp_by_card(&draw_two));
        assert_eq!(Ordering::Equal, draw_one.cmp_by_card(&draw_three));
        assert_eq!(Ordering::Less, draw_two.cmp_by_card(&draw_one));
    }
}
