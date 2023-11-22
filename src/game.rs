use macroquad::{
    math::{
        vec2,
        Rect
    },
    input::{
        KeyCode,
        is_key_pressed
    },
    rand::srand,
    shapes::draw_rectangle,
    color::{
        RED,
        GRAY
    },
    text::{
        draw_text,
        measure_text
    },
    miniquad::date
};

use crate::{
    tetromino::{
        Tetromino,
        SEGMENT_SIZE,
    },
    board::Board
};

#[derive(Debug)]
pub struct Game {
    score: u32,
    is_over: bool,
    current_piece: Tetromino,
    next_piece: Tetromino,
    held_piece: Option<Tetromino>,
    board: Board,
}
impl Game {
    fn test_print(&self) {
        println!("Current piece: {:?} at {}", self.current_piece.shape(), self.current_piece.position());
        if let Some(held_piece) = self.held_piece {
            println!("Held piece: {:?} at {}", held_piece.shape(), held_piece.position());
        }
        else {
            println!("Nothing held");
        }
    }
}
impl std::default::Default for Game {
    fn default() -> Self {
        srand(date::now() as u64);
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
        
        if is_key_pressed(KeyCode::Space) {
            self.test_print();
        }

        Ok(())
    }
    fn spawn_piece(&mut self) {
        self.current_piece = Tetromino::random();
        self.current_piece.set_position(self.board.top_middle());
    }
    fn swap_held_piece(&mut self) {
        self.current_piece.set_position(vec2(
            self.board.bounds().right() + 3.0 * SEGMENT_SIZE,
            self.board.bounds().top() + 2.0 * SEGMENT_SIZE
        ));

        match self.held_piece.take() {
            Some(held_piece) => {
                self.held_piece = Some(self.current_piece);
                self.current_piece = held_piece;
                self.current_piece.set_position(self.board.top_middle());
            },
            None => {
                self.held_piece = Some(self.current_piece);
                self.spawn_piece();
            },
        }
    }
    fn draw_held_piece(&self) {
        let hold_box = Rect {
            x: self.board.bounds().right() + SEGMENT_SIZE,
            y: self.board.bounds().top(),
            w: SEGMENT_SIZE * 4.0,
            h: SEGMENT_SIZE * 4.0,
        };

        draw_rectangle( hold_box.x, hold_box.y, hold_box.w, hold_box.h, GRAY);

        if let Some(held_piece) = self.held_piece {
            held_piece.draw()
        } else {
            let text = "NOTHING HELD";
            let text_dimensions = measure_text(text, None, 1, 15.0);
            draw_text(
                text,
                hold_box.center().x - text_dimensions.width / 2.0,
                hold_box.center().y,
                15.0,
                RED
            );
        }
    }
}