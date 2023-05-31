use crate::card::Card;
use crate::color::Color;
use crate::player::Player;
use crate::player_move::{PlayerId, PlayerMove};
use crate::rank::Rank;

use enum_iterator::all;
use itertools::iproduct;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Game {
    deck: Vec<Card>,
    seed: u64,
    players: HashMap<PlayerId, Player>,
    failed: Vec<(PlayerId, u32)>,
    round: u32,
    longest_war: u32,
}

impl Game {
    pub(crate) fn new(players_count: u8, seed: Option<u64>) -> Game {
        let ranks = all::<Rank>().collect::<Vec<_>>();
        let colors = all::<Color>().collect::<Vec<_>>();

        let players: HashMap<PlayerId, Player> = (1u8..=players_count)
            .map(|id| (id, Player::new(id)))
            .collect();

        let seed = if let Some(seed) = seed {
            seed
        } else {
            rand::random::<u64>()
        };

        Game {
            deck: iproduct!(ranks.into_iter(), colors.into_iter())
                .map(|(r, c)| Card::new(r, c))
                .collect::<Vec<_>>(),
            seed,
            players,
            failed: vec![],
            round: 0,
            longest_war: 0,
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    fn new_from_cards(hands: Vec<Vec<Card>>) -> Game {
        let players: HashMap<PlayerId, Player> = hands
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let player_id = (i + 1) as PlayerId;
                (player_id, Player::new_with_cards(player_id, item))
            })
            .collect();

        Game {
            deck: Vec::new(),
            seed: 0,
            players,
            failed: Vec::new(),
            round: 0,
            longest_war: 0,
        }
    }

    pub(crate) fn shuffle(&mut self) {
        let mut r = StdRng::seed_from_u64(self.seed);

        println!("Let's shuffle using seed {}", self.seed);
        self.deck.shuffle(&mut r);
    }

    pub(crate) fn deal(&mut self) {
        let quantity = self.deck.len() / self.players.len() + 1;
        for _ in 0..quantity {
            for (_, player) in self.players.iter_mut() {
                let card = self.deck.pop();
                match card {
                    Some(item) => player.add_card(item),
                    None => break,
                }
            }
        }
    }

    pub(crate) fn is_end(&self) -> bool {
        self.players.len() == 1
    }

    pub(crate) fn print_deck(&self) {
        print!("Deck: ");
        self.deck.iter().for_each(|card| print!("{card} "));
        println!();
    }

    pub(crate) fn print_players(&self) {
        self.players.iter().for_each(|(_, hand)| println!("{hand}"));
    }

    pub(crate) fn print_summary(&self) {
        self.players
            .iter()
            .for_each(|(id, _)| println!("Player {} won in round {}", id, self.round));
        self.failed
            .iter()
            .for_each(|(player, round)| println!("Player {} failed in {}", player, round));
        println!("Longest war: {}", self.longest_war);
    }

    pub(crate) fn play(&mut self) {
        while !self.is_end() {
            self.play_round();
            println!("Round {}", self.round);
            self.print_players();
        }
    }

    fn print_war(&self, players: &[PlayerId], final_cards: &Vec<PlayerMove>) {
        for player in players {
            print!("P{} \u{1F0A0}  ", player);
        }
        println!();
        print_player_moves(final_cards);
    }

    fn play_round(&mut self) {
        let mut result: Vec<PlayerMove> = Vec::new();
        let mut cards: Vec<PlayerMove> = self.get_players_moves();
        let mut winner = cards[0].player_id;
        let mut war_length = 0;

        print_player_moves(&cards);

        while let Some(players_in_war) = is_war(&cards) {
            result.append(&mut cards);
            let mut middle_cards = self.get_cards_for_players(&players_in_war);
            result.append(&mut middle_cards);
            cards = self.get_cards_for_players(&players_in_war);
            winner = cards[0].player_id;
            self.print_war(&players_in_war, &cards);
            war_length += 1;
        }

        if war_length > self.longest_war {
            self.longest_war = war_length;
        }

        println!("Winner {winner}");
        result.append(&mut cards);

        for card_move in result {
            self.players
                .get_mut(&winner)
                .unwrap()
                .add_card(card_move.card);
        }

        let failed_players: Vec<PlayerId> = self
            .players
            .iter()
            .filter(|(_, p)| p.failed())
            .map(|(id, _)| *id)
            .collect();

        for id in failed_players {
            self.players.remove(&id);
            self.failed.push((id, self.round))
        }

        self.round += 1;
    }

    fn get_players_moves(&mut self) -> Vec<PlayerMove> {
        let all_players = self.players.keys().copied().collect::<Vec<PlayerId>>();
        self.get_cards_for_players(&all_players)
    }

    fn get_cards_for_players(&mut self, players_id: &[PlayerId]) -> Vec<PlayerMove> {
        let mut cards: Vec<PlayerMove> = Vec::new();

        for id in players_id.iter() {
            if let Some(player) = self.players.get_mut(id) {
                if let Some(player_move) = player.get_card() {
                    cards.push(player_move)
                }
            }
        }

        cards.sort_by(|a, b| b.cmp_by_card(a));

        cards
    }
}

fn print_player_moves(cards: &Vec<PlayerMove>) {
    for card in cards {
        print!("P{} {} ", card.player_id, card.card);
    }
    println!();
}

fn is_war(table: &Vec<PlayerMove>) -> Option<Vec<PlayerId>> {
    if table.len() > 1 {
        let first_card = &table[0].card;
        let first_player = table[0].player_id;
        let mut players_in_war: Vec<PlayerId> = vec![];

        players_in_war.push(first_player);

        for player_move in table.iter().skip(1) {
            if first_card == &player_move.card {
                players_in_war.push(player_move.player_id);
            }
        }

        if players_in_war.len() > 1 {
            Some(players_in_war)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_players_moves() {
        let hands = vec![
            vec![Card::new(Rank::Ace, Color::Clubs)],
            vec![Card::new(Rank::Ten, Color::Hearts)],
            vec![Card::new(Rank::Seven, Color::Clubs)],
        ];

        let mut cards: Vec<PlayerMove> = hands
            .iter()
            .enumerate()
            .map(|(i, cards)| PlayerMove::new((i + 1) as PlayerId, cards[0]))
            .collect();
        let mut game = Game::new_from_cards(hands);

        cards.sort_by(|a, b| b.cmp_by_card(a));

        assert!(itertools::equal(cards, game.get_players_moves()));
    }

    #[test]
    fn test_is_war() {
        assert_eq!(
            Some(vec![1, 3, 4]),
            is_war(&vec![
                PlayerMove::new(1, Card::new(Rank::Ace, Color::Spades)),
                PlayerMove::new(3, Card::new(Rank::Ace, Color::Diamonds)),
                PlayerMove::new(4, Card::new(Rank::Ace, Color::Hearts)),
                PlayerMove::new(2, Card::new(Rank::Ten, Color::Clubs)),
            ])
        );
    }

    #[test]
    fn test_is_war_no_war() {
        assert_eq!(
            None,
            is_war(&vec![
                PlayerMove::new(3, Card::new(Rank::Ace, Color::Diamonds)),
                PlayerMove::new(1, Card::new(Rank::Two, Color::Clubs)),
                PlayerMove::new(2, Card::new(Rank::Ten, Color::Hearts)),
            ])
        );
    }

    #[test]
    fn test_is_war_not_first() {
        assert_eq!(
            None,
            is_war(&vec![
                PlayerMove::new(3, Card::new(Rank::Ace, Color::Diamonds)),
                PlayerMove::new(1, Card::new(Rank::Ten, Color::Clubs)),
                PlayerMove::new(2, Card::new(Rank::Ten, Color::Hearts)),
            ])
        );
    }

    #[test]
    fn test_is_war_one_item() {
        assert_eq!(
            None,
            is_war(&vec![PlayerMove::new(
                1,
                Card::new(Rank::King, Color::Spades)
            )])
        );
    }

    #[test]
    fn test_is_war_empty_table() {
        assert_eq!(None, is_war(&Vec::new()));
    }
}
