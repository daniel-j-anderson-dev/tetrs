mod game;
mod square;
mod tetromino;
mod board;

use macroquad::{
    self,
    window::{
        clear_background,
        next_frame
    },
    input::KeyCode,
    color::BLACK, input::is_key_pressed
};

use game::Game;

#[macroquad::main("tetrs")]
async fn main() {
    let mut game = Game::default();

    loop {
        clear_background(BLACK);

        let _ = game.update();

        if is_key_pressed(KeyCode::Enter) {
            game = Game::default();
        }

        next_frame().await;
    }
}

