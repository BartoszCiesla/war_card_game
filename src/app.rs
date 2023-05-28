use crate::args::Args;
use crate::game::Game;

use clap::Parser;

pub(crate) struct App {
    args: Args,
    game: Game,
}

impl App {
    pub(crate) fn new() -> App {
        let args = Args::parse();
        let game = Game::new(2);

        App { args, game }
    }

    pub(crate) fn run(&mut self) {
        println!("Starting game!");
        self.game.print_deck();
        let seed = if let Some(seed) = self.args.get_seed() {
            seed
        } else {
            rand::random::<u64>()
        };

        println!("Let's shuffle using seed {seed}");
        self.game.shuffle(seed);
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
