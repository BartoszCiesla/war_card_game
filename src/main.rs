mod app;
mod args;
mod card;
mod color;
mod game;
mod player;
mod player_move;
mod rank;
mod unicode;

// use crate::app::App;

fn main() {
    let mut app = app::App::new();

    app.run()
}
