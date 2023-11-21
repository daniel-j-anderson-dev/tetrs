use macroquad::{math::vec2, input::{KeyCode, is_key_pressed}, rand, shapes::draw_rectangle, color::{LIGHTGRAY, RED, GREEN}, text::{draw_text, measure_text}};

use crate::{
    tetromino::{Tetromino, SEGMENT_SIZE},
    board::Board
};

pub struct Game {
    score: u32,
    is_over: bool,
    current_piece: Tetromino,
    next_piece: Tetromino,
    held_piece: Option<Tetromino>,
    board: Board,
}
impl std::default::Default for Game {
    fn default() -> Self {
        rand::srand(macroquad::miniquad::date::now() as u64);
        Game {
            score: 0,
            is_over: false,
            current_piece: Tetromino::random(),
            next_piece: Tetromino::random(),
            held_piece: None,
            board: Board::default(),
        }
    }
}
impl Game {
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
        if is_key_pressed(KeyCode::C) {
            self.swap_held_piece();
        }

        self.current_piece.move_down();
        
        self.board.keep_in_bounds(&mut self.current_piece);
        
        
        self.board.draw();
        self.draw_held_piece();
        self.current_piece.draw();
        
        Ok(())
    }
    fn spawn_piece(&mut self) {
        let bounds = self.board.bounds();
        let top_middle = vec2((bounds.right() - bounds.left()) / 2.0, bounds.top() + SEGMENT_SIZE);

        self.current_piece = Tetromino::random();
        self.current_piece.set_position(top_middle);
    }
    fn swap_held_piece(&mut self) {
        self.current_piece.set_position(vec2(self.board.bounds().right() + 2.0 * SEGMENT_SIZE, self.board.bounds().top() + SEGMENT_SIZE / 2.0));

        match self.held_piece.take() {
            Some(held_piece) => {
                self.held_piece = Some(self.current_piece);
                self.current_piece = held_piece;
            },
            None => {
                self.held_piece = Some(self.current_piece);
                self.spawn_piece();
            },
        }
    }
    fn draw_held_piece(&self) {
        draw_rectangle(
            self.board.bounds().right() + SEGMENT_SIZE,
            self.board.bounds().top(),
            SEGMENT_SIZE * 4.0,
            SEGMENT_SIZE * 4.0,
            LIGHTGRAY,
        );
        if let Some(held_piece) = self.held_piece {
            held_piece.draw();
        } else {
            let text = "NOTHING HELD";
            // let text_width = measure_text(text, font, font_size, font_scale)
            draw_text("NOTHING HELD", self.board.bounds().right() + 2.0 * SEGMENT_SIZE, self.board.bounds().top() + 2.0 * SEGMENT_SIZE, 20.0, RED);
        }
    }
}