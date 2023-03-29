mod card;
mod color;
mod game;
mod player;
mod player_move;
mod rank;
mod unicode;

use crate::game::Game;

fn main() {
    let mut game = Game::new(2);
    let seed = rand::random::<u64>();

    println!("Starting game!");
    game.print_deck();
    println!("Let's shuffle using seed {seed}");
    game.shuffle(seed);
    game.print_deck();
    println!("Let's deal!");
    game.deal();
    println!("Starting setup:");
    game.print_players();
    game.play();
    println!("Game over!");
    game.print_summary();
}
