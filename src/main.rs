mod card;
mod color;
mod game;
mod player;
mod player_move;
mod rank;
mod unicode;

use crate::color::Color;
use crate::game::Game;
use crate::rank::Rank;

use enum_iterator::all;
// use queues::Queue;

fn main() {
    let mut game = Game::new();

    println!("Starting game!");
    println!("{game}",);
    println!("Let's deal!");
    game.deal();
    println!("{game}");

    for color in all::<Color>() {
        print!("{color}");
    }

    for rank in all::<Rank>() {
        print!("{rank}");
    }
}
