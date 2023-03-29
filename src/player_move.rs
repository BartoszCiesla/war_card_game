use crate::card::Card;

use std::cmp::Ordering;

pub(crate) type PlayerId = u8;

#[derive(Debug)]
pub(crate) struct PlayerMove {
    pub(crate) player_id: PlayerId,
    pub(crate) card: Card,
}

impl PlayerMove {
    pub(crate) fn new(player_id: PlayerId, card: Card) -> Self {
        PlayerMove { player_id, card }
    }

    pub(crate) fn cmp_by_card(&self, other: &Self) -> Ordering {
        self.card.cmp(&other.card)
    }
}

impl PartialEq for PlayerMove {
    fn eq(&self, other: &Self) -> bool {
        self.player_id.eq(&other.player_id) && self.card.eq(&other.card)
    }
}

impl Eq for PlayerMove {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::color::Color;
    use crate::rank::Rank;

    #[test]
    fn test_cmp_by_card() {
        let move_one = PlayerMove::new(1, Card::new(Rank::Ace, Color::Spades));
        let move_two = PlayerMove::new(2, Card::new(Rank::Ten, Color::Hearts));
        let move_three = PlayerMove::new(3, Card::new(Rank::Ace, Color::Diamonds));

        assert_eq!(Ordering::Greater, move_one.cmp_by_card(&move_two));
        assert_eq!(Ordering::Equal, move_one.cmp_by_card(&move_three));
        assert_eq!(Ordering::Less, move_two.cmp_by_card(&move_one));
    }

    #[test]
    fn test_eq() {
        let player_move = PlayerMove::new(1, Card::new(Rank::Ace, Color::Spades));

        assert!(player_move.eq(&PlayerMove::new(1, Card::new(Rank::Ace, Color::Spades))));
        assert!(player_move.eq(&PlayerMove::new(1, Card::new(Rank::Ace, Color::Diamonds))));
        assert!(!player_move.eq(&PlayerMove::new(1, Card::new(Rank::Ten, Color::Hearts))));
        assert!(!player_move.eq(&PlayerMove::new(2, Card::new(Rank::Ace, Color::Hearts))));
        assert!(!player_move.eq(&PlayerMove::new(2, Card::new(Rank::Six, Color::Spades))));
    }
}
