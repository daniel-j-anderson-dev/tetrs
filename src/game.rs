use macroquad::math::vec2;

use crate::{
    tetromino::{Tetromino, SEGMENT_SIZE},
    board::Board
}
;
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
        Self {
            score: 0,
            is_over: false,
            current_piece: Tetromino::random(),
            held_piece: None,
            next_piece: Tetromino::random(),
            board: Board::default(),
        }
    }
}
impl Game {
    fn spawn_piece(&mut self) {
        let bounds = self.board.bounds();
        let top_middle = vec2((bounds.right() - bounds.left()) / 2.0, bounds.top() + SEGMENT_SIZE);
        self.current_piece = Tetromino::random();
        self.current_piece.set_position(top_middle);
    }
    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}