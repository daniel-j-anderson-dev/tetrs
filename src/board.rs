use macroquad::{
    color::GRAY,
    math::{
        Rect,
        Vec2, vec2,
    },
    shapes::draw_rectangle,
    window::{
        screen_width,
        screen_height,
    },
};

use crate::tetromino::{
    Tetromino,
    SEGMENT_SIZE
};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Debug)]
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

    pub fn keep_in_bounds(&self, piece: &mut Tetromino) {
        let piece_top = piece.top();
        let piece_left = piece.left();
        let piece_bottom = piece.bottom();
        let piece_right = piece.right();

        if piece_top < self.bounds.top() {
            piece.add_position(Vec2::Y * (self.bounds.top() - piece_top));
        }

        if piece_left < self.bounds.left() {
            piece.add_position(Vec2::X * (self.bounds.left() - piece_left));
        }

        if piece_bottom > self.bounds.bottom() {
            piece.add_position(Vec2::Y * (self.bounds.bottom() - piece_bottom));
        }

        if piece_right > self.bounds.right() {
            piece.add_position(Vec2::X * (self.bounds.right() - piece_right));
        }
    }
    pub fn clear_lines(&mut self) -> usize {
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

        for piece in self.pieces.iter() {
            piece.draw();
        }
    }

}

impl Board {
    pub fn top_middle(&self) -> Vec2 {
        vec2(
            self.bounds.x + self.bounds.w / 2.0,
            self.bounds.y + SEGMENT_SIZE
        )
    }
    pub fn bounds(&self) -> Rect {
        self.bounds
    }
}

