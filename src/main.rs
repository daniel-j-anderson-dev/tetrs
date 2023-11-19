mod game;
mod square;
mod tetromino;

use macroquad::prelude::*;
pub use tetromino::Tetromino;

#[macroquad::main("tetrs")]
async fn main() {
    let mut pieces = tetromino::all_shapes();

    let mut piece = &mut pieces[0];

    loop {
        clear_background(WHITE);

        // select piece
        if is_key_pressed(KeyCode::Key1) { piece = &mut pieces[0] }
        if is_key_pressed(KeyCode::Key2) { piece = &mut pieces[1] }
        if is_key_pressed(KeyCode::Key3) { piece = &mut pieces[2] }
        if is_key_pressed(KeyCode::Key4) { piece = &mut pieces[3] }
        if is_key_pressed(KeyCode::Key5) { piece = &mut pieces[4] }
        if is_key_pressed(KeyCode::Key6) { piece = &mut pieces[5] }
        if is_key_pressed(KeyCode::Key7) { piece = &mut pieces[6] }

        // rotate the piece
        if is_key_pressed(KeyCode::Z) {
            piece.rotate_right();
        }
        if is_key_down(KeyCode::Z) {
            draw_text("Z: Clockwise Rotation", screen_width()  / 2.5, screen_height() / 2.5, 10.0, BLACK);
        }
        if is_key_pressed(KeyCode::X) {
            piece.rotate_left();
        }
        if is_key_down(KeyCode::X) {
            draw_text("X: Counter-clockwise Rotation", screen_width() / 2.5, screen_height() / 2.5, 10.0, BLACK);
        }

        // draw piece
        piece.draw();
        piece.draw_center(PINK);

        next_frame().await;
    }
}
