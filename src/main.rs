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
    let mut board = Board::default();
    let mut falling_piece = Tetromino::random();
    let bounds = board.bounds();
    let x_mid = (bounds.right() - bounds.left()) / 2.0;
    falling_piece.add_position(vec2(bounds.left() + x_mid, bounds.top() + SEGMENT_SIZE));


    loop {
        clear_background(BLUE);

        if is_key_pressed(KeyCode::Left) {
            falling_piece.move_left();
        }
        if is_key_pressed(KeyCode::Right) {
            falling_piece.move_right();
        }
        if is_key_pressed(KeyCode::X) {
            falling_piece.rotate(false);
        }
        if is_key_pressed(KeyCode::Z) {
            falling_piece.rotate(true);
        }

        board.keep_in_bounds(&mut falling_piece);

        board.draw();
        falling_piece.draw();

        next_frame().await;
    }
}

// #[macroquad::main("tetrs")]
// async fn main() {
//     let mut board = Board::default();
//     let mut pieces = tetromino::all_shapes();

//     let mut piece = &mut pieces[0];

//     loop {
//         clear_background(WHITE);

//         // select piece
//         if is_key_pressed(KeyCode::Key1) { piece = &mut pieces[0] }
//         if is_key_pressed(KeyCode::Key2) { piece = &mut pieces[1] }
//         if is_key_pressed(KeyCode::Key3) { piece = &mut pieces[2] }
//         if is_key_pressed(KeyCode::Key4) { piece = &mut pieces[3] }
//         if is_key_pressed(KeyCode::Key5) { piece = &mut pieces[4] }
//         if is_key_pressed(KeyCode::Key6) { piece = &mut pieces[5] }
//         if is_key_pressed(KeyCode::Key7) { piece = &mut pieces[6] }

//         // rotate the piece
//         if is_key_pressed(KeyCode::Z) {
//             piece.rotate(true);
//         }
//         if is_key_down(KeyCode::Z) {
//             draw_text("Z: Clockwise Rotation", screen_width()  / 2.5, screen_height() / 2.5, 10.0, BLACK);
//         }
//         if is_key_pressed(KeyCode::X) {
//             piece.rotate(false);
//         }
//         if is_key_down(KeyCode::X) {
//             draw_text("X: Counter-clockwise Rotation", screen_width() / 2.5, screen_height() / 2.5, 10.0, BLACK);
//         }




//         // draw piece
//         piece.draw();
//         piece.draw_center(PINK);

//         next_frame().await;
//     }
// }
