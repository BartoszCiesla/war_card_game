use crate::args::{Args, Output};
use crate::game::Game;

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

pub(crate) struct App {
    args: Args,
}

impl App {
    pub(crate) fn new() -> App {
        let args = Args::parse();

        if let Some(range) = args.get_seed() {
            let output = args.get_output();
            if range.len() > 1 && output != Output::OneLine {
                let mut cmd = Args::command();
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    format!("Can't use {} output level with seed range.", output),
                )
                .exit();
            }
        }

        App { args }
    }

    fn play_game(&mut self, seed: Option<u64>) {
        let mut game = Game::new(2, seed, self.args.get_output());
        game.init();
        game.deal();
        game.start();
        game.play();
        game.finish();
    }

    pub(crate) fn run(&mut self) {
        match self.args.get_seed() {
            None => {
                self.play_game(None);
            }
            Some(range) => {
                range.into_iter().for_each(|seed| {
                    self.play_game(Some(seed));
                });
            }
        }
    }
}
