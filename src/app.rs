use crate::args::Args;
use crate::game::Game;

use clap::Parser;

pub(crate) struct App {
    game: Game,
}

impl App {
    pub(crate) fn new() -> App {
        let args = Args::parse();
        let game = Game::new(2, args.get_seed(), args.get_output());

        App { game }
    }

    pub(crate) fn run(&mut self) {
        self.game.init();
        self.game.deal();
        self.game.start();
        self.game.play();
        self.game.finish();
    }
}
