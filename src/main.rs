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
    color::{
        BLACK,
        RED
    },
    text::draw_text
};
use game::Game;

#[macroquad::main("tetrs")]
async fn main() {
    let mut game = Game::default();

    loop {
        clear_background(BLACK);

        game.update();

        next_frame().await;
    }
}

