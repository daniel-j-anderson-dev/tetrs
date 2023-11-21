use macroquad::{math::vec2, input::{KeyCode, is_key_pressed}};

use crate::{
    tetromino::{Tetromino, SEGMENT_SIZE},
    board::Board
};

pub struct Game {
    score: u32,
    is_over: bool,
    current_piece: Tetromino,
    held_piece: Option<Tetromino>,
    next_piece: Tetromino,
    board: Board
}
impl std::default::Default for Game {
    fn default() -> Self {
        let mut game = Game {
            score: 0,
            is_over: false,
            current_piece: Tetromino::random(),
            held_piece: None,
            next_piece: Tetromino::random(),
            board: Board::default(),
        };
        game
    }
}
impl Game {
    fn spawn_piece(&mut self) {
        let bounds = self.board.bounds();
        let top_middle = vec2((bounds.right() - bounds.left()) / 2.0, bounds.top() + SEGMENT_SIZE);
        self.current_piece = Tetromino::random();
        self.current_piece.add_position(top_middle);
    }
    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if is_key_pressed(KeyCode::Left) {
            self.current_piece.move_left();
        }
        if is_key_pressed(KeyCode::Right) {
            self.current_piece.move_right();
        }
        if is_key_pressed(KeyCode::X) {
            self.current_piece.rotate(false);
        }
        if is_key_pressed(KeyCode::Z) {
            self.current_piece.rotate(true);
        }

        self.current_piece.move_down();
        
        self.board.keep_in_bounds(&mut self.current_piece);
        

        self.board.draw();
        self.current_piece.draw();

        Ok(())
    }
}