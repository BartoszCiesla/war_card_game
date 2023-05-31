use crate::args::Args;
use crate::game::Game;

use clap::Parser;

pub(crate) struct App {
    game: Game,
}

impl App {
    pub(crate) fn new() -> App {
        let args = Args::parse();
        let game = Game::new(2, args.get_seed());

        App { game }
    }

    pub(crate) fn run(&mut self) {
        println!("Starting game!");
        self.game.print_deck();
        self.game.shuffle();
        self.game.print_deck();
        println!("Let's deal!");
        self.game.deal();
        println!("Starting setup:");
        self.game.print_players();
        self.game.play();
        println!("Game over!");
        self.game.print_summary();
    }
}
