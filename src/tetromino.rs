use macroquad::{
    prelude::*,
    rand::rand,
};

use crate::square::Square;

pub const SEGMENT_SIZE: f32 = 20.0;
pub const SEGMENT_BORDER_COLOR: Color = BLACK;
pub const SEGMENT_BORDER_SIZE: f32 = 1.0;

/// https://en.wikipedia.org/wiki/Tetromino#One-sided_tetrominoes
#[derive(Clone, Copy)]
pub enum Shape {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}


pub struct Tetromino {
    position: Vec2, // center as shown in `SRS-pieces.webp`
    segments: [Square; 4],
    shape: Shape,
}

// constructors
impl Tetromino {
    pub fn new_I(x: f32, y: f32) -> Tetromino {
        let segment_0 = Square::new(x -2.0 * SEGMENT_SIZE, y - SEGMENT_SIZE, SEGMENT_SIZE);
        let segment_1 = Square::new(segment_0.right(), segment_0.top(), SEGMENT_SIZE);
        let segment_2 = Square::new(segment_1.right(), segment_0.top(), SEGMENT_SIZE);
        let segment_3 = Square::new(segment_2.right(), segment_0.top(), SEGMENT_SIZE);

        Tetromino {
            shape: Shape::I,
            position: vec2(x, y),
            segments: [segment_0, segment_1, segment_2, segment_3],
        }
    }
    pub fn new_O(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(x - SEGMENT_SIZE, y - SEGMENT_SIZE, SEGMENT_SIZE);
        let square_1 = Square::new(square_0.right(), square_0.top(),               SEGMENT_SIZE);
        let square_2 = Square::new(square_0.left(),  square_0.bottom(), SEGMENT_SIZE);
        let square_3 = Square::new(square_1.left(),  square_1.bottom(), SEGMENT_SIZE);

        Tetromino {
            shape: Shape::O,
            position: vec2(x, y),
            segments: [square_0, square_1, square_2, square_3],
        }
    }
    pub fn new_T(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(x -0.5 * SEGMENT_SIZE, y -1.5 * SEGMENT_SIZE, SEGMENT_SIZE);
        let square_1 = Square::new(square_0.left(), square_0.bottom(), SEGMENT_SIZE);
        let square_2 = Square::new(square_1.right(), square_1.top(), SEGMENT_SIZE);
        let square_3 = Square::new(square_1.left() - SEGMENT_SIZE,  square_1.top(), SEGMENT_SIZE);

        Tetromino {
            shape: Shape::T,
            position: vec2(x, y),
            segments: [square_0, square_1, square_2, square_3],
        }
    }
    pub fn new_J(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(x -1.5 * SEGMENT_SIZE, y -1.5 * SEGMENT_SIZE, SEGMENT_SIZE);
        let square_1 = Square::new(square_0.left(), square_0.bottom(), SEGMENT_SIZE);
        let square_2 = Square::new(square_1.right(), square_1.top(), SEGMENT_SIZE);
        let square_3 = Square::new(square_2.right(), square_2.top(), SEGMENT_SIZE);

        Tetromino {
            shape: Shape::J,
            position: vec2(x, y),
            segments: [square_0, square_1, square_2, square_3],
        }
    }
    pub fn new_L(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(x - 1.5 * SEGMENT_SIZE, y - 0.5 * SEGMENT_SIZE, SEGMENT_SIZE);
        let square_1 = Square::new(square_0.right(), square_0.top(),SEGMENT_SIZE);
        let square_2 = Square::new(square_1.right(), square_1.top(), SEGMENT_SIZE);
        let square_3 = Square::new(square_2.left(), square_2.top() - SEGMENT_SIZE, SEGMENT_SIZE);

        Tetromino { 
            position: vec2(x, y),
            segments: [square_0, square_1, square_2, square_3],
            shape: Shape::L
        }
    }
    pub fn new_S(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(x - 0.5 * SEGMENT_SIZE, y - 1.5 * SEGMENT_SIZE, SEGMENT_SIZE);
        let square_1 = Square::new(square_0.right(), square_0.top(), SEGMENT_SIZE);
        let square_2 = Square::new(square_0.left() - square_0.w, square_0.bottom(), SEGMENT_SIZE);
        let square_3 = Square::new(square_2.right(), square_1.bottom(), SEGMENT_SIZE);

        Tetromino { 
            position: vec2(x, y),
            segments: [square_0, square_1, square_2, square_3],
            shape: Shape::S,
        }
    }
    pub fn new_Z(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(x - 1.5 * SEGMENT_SIZE, y - 1.5 * SEGMENT_SIZE, SEGMENT_SIZE);
        let square_1 = Square::new(square_0.right(), square_0.top(), SEGMENT_SIZE);
        let square_2 = Square::new(square_1.left(), square_1.bottom(), SEGMENT_SIZE);
        let square_3 = Square::new(square_2.right(), square_1.bottom(), SEGMENT_SIZE);


        Tetromino {
            shape: Shape::Z,
            position: vec2(x, y),
            segments: [square_0, square_1, square_2, square_3],
        }
    }
    pub fn random(x: f32, y: f32) -> Tetromino {
        match rand() % 7 {
            0 => Tetromino::new_I(x, y),
            1 => Tetromino::new_O(x, y),
            2 => Tetromino::new_T(x, y),
            3 => Tetromino::new_J(x, y),
            4 => Tetromino::new_L(x, y),
            5 => Tetromino::new_S(x, y),
            6 => Tetromino::new_Z(x, y),
            _ => Tetromino::new_I(x, y),
        }
    }
    pub fn random_at_origin() -> Tetromino {
        Tetromino::random(0.0, 0.0)
    }
}

// getters
impl Tetromino {
    pub fn shape(&self) -> Shape {
        self.shape
    }
    pub fn squares(&self) -> &[Square] {
        &self.segments
    }
}

// setters
impl Tetromino {
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
}

// actions
impl Tetromino {
    pub fn move_left(&mut self) {
        for sq in self.segments.iter_mut() {
            sq.x -= sq.w;
        }
    }
    pub fn move_right(&mut self) {
        for sq in self.segments.iter_mut() {
            sq.x += sq.w;
        }
    }
    pub fn move_down(&mut self) {
        for sq in self.segments.iter_mut() {
            sq.y += sq.h;
        }
    }
    pub fn rotate_ccw(&mut self) {
        for square in self.segments.iter_mut() {
            let segment_center = vec2(
                square.x + 0.5 * SEGMENT_SIZE,
                square.y + 0.5 * SEGMENT_SIZE,
            );

            // account for position
            let difference = segment_center - self.position;

            // Perform a 90-degree counterclockwise rotation (x, y) -> (-y, x)
            let rotated = vec2(-difference.y, difference.x) + self.position; // account for position

            // Set the square to the roated position acounting for the center
            square.x = rotated.x - 0.5 * SEGMENT_SIZE;
            square.y = rotated.y - 0.5 * SEGMENT_SIZE;
        }
    }
    pub fn rotate_cw(&mut self) {
        for square in self.segments.iter_mut() {
            let segment_center = vec2( // square st
                square.x + 0.5 * SEGMENT_SIZE,
                square.y + 0.5 * SEGMENT_SIZE,
            );

            let difference = segment_center - self.position;

            // Perform a 90-degree clockwise rotation (x, y) -> (y, -x)
            let rotated = vec2(difference.y, -difference.x) + self.position; // account for position

            // Update the square's position based on the new relative coordinates
            square.x = rotated.x - 0.5 * SEGMENT_SIZE;
            square.y = rotated.y - 0.5 * SEGMENT_SIZE;
        }
    }

    pub fn draw(&self) {
        let interior_color = match self.shape {
            Shape::I => BLUE,
            Shape::O => LIGHTGRAY,
            Shape::T => YELLOW,
            Shape::J => PURPLE,
            Shape::L => Color::from_rgba(0, 255, 255, 255), // CYAN
            Shape::S => GREEN,
            Shape::Z => RED,
        };

        for sq in self.segments.iter() {
            // draw border
            draw_rectangle(
                sq.x,
                sq.y,
                sq.w,
                sq.h,
                SEGMENT_BORDER_COLOR
            );

            // draw interior
            draw_rectangle(
                sq.x + SEGMENT_BORDER_SIZE,
                sq.y + SEGMENT_BORDER_SIZE,
                sq.w - 2.0 * SEGMENT_BORDER_SIZE,
                sq.h - 2.0 * SEGMENT_BORDER_SIZE,
                interior_color
            );
        }
    }
    pub fn draw_center(&self, color: Color) {
        draw_circle(self.position.x, self.position.y, 3.5, PINK)
    }
}

pub fn all_shapes() -> [Tetromino; 7] {
    let origin = vec2(screen_width(), screen_height())/2.0;

    let i = Tetromino::new_I(origin.x, origin.y);
    let o = Tetromino::new_O(origin.x, origin.y);
    let t = Tetromino::new_T(origin.x, origin.y);
    let j = Tetromino::new_J(origin.x, origin.y);
    let l = Tetromino::new_L(origin.x, origin.y);
    let s = Tetromino::new_S(origin.x, origin.y);
    let z = Tetromino::new_Z(origin.x, origin.y);

    [i, o, t, j, l, s, z]
}