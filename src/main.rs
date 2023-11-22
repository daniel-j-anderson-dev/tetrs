mod game;
mod square;
mod tetromino;
mod board;

use game::Game;
use macroquad::{
    self,
    window::{
        clear_background,
        next_frame},
    color::{BLACK, RED}, input::{mouse_position, is_mouse_button_pressed, MouseButton}, text::draw_text
};

#[macroquad::main("tetrs")]
async fn main() {
    let mut game = Game::default();

    loop {
        clear_background(BLACK);
        let (mut mouse_x, mut mouse_y) = mouse_position();

        let text = format!("Mouse: ({}, {})", mouse_x, mouse_y);
        draw_text(&text, 20.0, 20.0, 20.0, RED);

        game.update();

        next_frame().await;
    }
}

