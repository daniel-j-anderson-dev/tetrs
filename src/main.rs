mod game;
mod square;
mod tetromino;
mod board;

use board::Board;
use game::Game;
use macroquad::prelude::*;
use tetromino::SEGMENT_SIZE;
pub use tetromino::Tetromino;

#[macroquad::main("tetrs")]
async fn main() {
    let mut game = Game::default();

    loop {
        clear_background(BLACK);

        game.update();

        next_frame().await;
    }
}

