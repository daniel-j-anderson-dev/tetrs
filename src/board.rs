use macroquad::{color::{Color, LIGHTGRAY, GRAY}, math::{Vec2,vec2, Rect}, window::{screen_width, screen_height}, input::is_key_pressed, miniquad::KeyCode, shapes::draw_rectangle};

use crate::{Tetromino, tetromino::SEGMENT_SIZE};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

fn center() -> Vec2 {
    vec2(screen_width(), screen_height()) / 2.0
}

pub struct Board {
    pieces: Vec<Tetromino>,
    bounds: Rect,
}
impl std::default::Default for Board {
    fn default() -> Self {
        Board { 
            pieces: Vec::new(),
            bounds: Rect {
                x: (screen_width() - (SEGMENT_SIZE * BOARD_WIDTH as f32)) / 2.0,
                y: (screen_height() - (SEGMENT_SIZE * BOARD_HEIGHT as f32)) / 2.0,
                w: SEGMENT_SIZE * BOARD_WIDTH as f32,
                h: SEGMENT_SIZE * BOARD_HEIGHT as f32,
            }
        }
    }
}
impl Board {
    // if piece is out of bounds this returns the portion that is out of bounds
    // otherwise None
    pub fn check_is_bounds(&self, piece: Tetromino) -> Option<Rect> {
        for square in piece.squares() {
            if square.bottom() > self.bounds.bottom() {
                return Some(Rect { 
                    x: square.left(),
                    y: self.bounds.bottom(),
                    w: square.w,
                    h: (square.bottom() - self.bounds.bottom()).abs(),
                });
            }
            if square.top() < self.bounds.top() {
                return Some(Rect { 
                    x: square.left(),
                    y: square.top(),
                    w: square.w,
                    h: (square.top() - self.bounds.top()).abs(),
                });
            }
            if  square.left() < self.bounds.left() {
                return Some(Rect { 
                    x: square.left(),
                    y: square.top(),
                    w: (square.left() - self.bounds.left()).abs(),
                    h: square.h,
                });
            }
            if square.right() > self.bounds.right() {
                return Some(Rect { 
                    x: self.bounds.right(),
                    y: square.top(),
                    w: (square.right() - self.bounds.right()).abs(),
                    h: square.h,
                });
            }
        }

        None
    }
    pub fn clear_lines(&mut self) -> Option<usize> {
        todo!();
    }

    pub fn add_piece(&mut self, piece: Tetromino) {
        self.pieces.push(piece);
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            GRAY,
        );
        draw_rectangle(
            self.bounds.x + SEGMENT_SIZE,
            self.bounds.y + SEGMENT_SIZE,
            self.bounds.w - SEGMENT_SIZE,
            self.bounds.h - SEGMENT_SIZE,
            GRAY,
        );

        for piece in self.pieces.iter() {
            piece.draw();
        }
    }

}

impl Board {
    pub fn bounds(&self) -> Rect {
        self.bounds
    }
}

