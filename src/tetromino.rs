use macroquad::{
    color::{
        Color,
        BLACK, BLUE, YELLOW, PURPLE, GREEN, RED, LIGHTGRAY, PINK,
    },
    math::{
        Rect,
        Vec2, vec2,
    },
    shapes::{
        draw_rectangle,
        draw_circle
    },
    window::{
        screen_width,
        screen_height,
    },
    rand::rand,
};

use crate::square::Square;

pub const SEGMENT_SIZE: f32 = 20.0;
pub const SEGMENT_BORDER_COLOR: Color = BLACK;
pub const SEGMENT_BORDER_SIZE: f32 = 1.0;

/// https://en.wikipedia.org/wiki/Tetromino#One-sided_tetrominoes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}
impl Shape {
    pub fn segments(&self, position: Vec2) -> [Square; 4] {
        let segment_0;
        let segment_1;
        let segment_2;
        let segment_3;
        
        match self {
            Shape::I => {
                segment_0 = Square::new(position.x - 2.0 * SEGMENT_SIZE, position.y - 1.0 * SEGMENT_SIZE, SEGMENT_SIZE);
                segment_1 = Square::new(segment_0.right(), segment_0.top(), SEGMENT_SIZE);
                segment_2 = Square::new(segment_1.right(), segment_0.top(), SEGMENT_SIZE);
                segment_3 = Square::new(segment_2.right(), segment_0.top(), SEGMENT_SIZE);
            },
            Shape::O => {
                segment_0 = Square::new(position.x - 1.0 * SEGMENT_SIZE, position.y - 1.0 * SEGMENT_SIZE, SEGMENT_SIZE);
                segment_1 = Square::new(segment_0.right(), segment_0.top(), SEGMENT_SIZE);
                segment_2 = Square::new(segment_0.left(), segment_0.bottom(), SEGMENT_SIZE);
                segment_3 = Square::new(segment_0.right(), segment_0.bottom(), SEGMENT_SIZE);
            },
            Shape::T => {
                segment_0 = Square::new(position.x -0.5 * SEGMENT_SIZE, position.y -1.5 * SEGMENT_SIZE, SEGMENT_SIZE);
                segment_1 = Square::new(segment_0.left(), segment_0.bottom(), SEGMENT_SIZE);
                segment_2 = Square::new(segment_1.right(), segment_1.top(), SEGMENT_SIZE); 
                segment_3 = Square::new(segment_1.left() - SEGMENT_SIZE,  segment_1.top(), SEGMENT_SIZE); 
            },
            Shape::J => {
                segment_0 = Square::new(position.x -1.5 * SEGMENT_SIZE, position.y -1.5 * SEGMENT_SIZE, SEGMENT_SIZE); 
                segment_1 = Square::new(segment_0.left(), segment_0.bottom(), SEGMENT_SIZE);
                segment_2 = Square::new(segment_1.right(), segment_1.top(), SEGMENT_SIZE);
                segment_3 = Square::new(segment_2.right(), segment_2.top(), SEGMENT_SIZE);
            },
            Shape::L => {
                segment_0 = Square::new(position.x - 1.5 * SEGMENT_SIZE, position.y - 0.5 * SEGMENT_SIZE, SEGMENT_SIZE); 
                segment_1 = Square::new(segment_0.right(), segment_0.top(),SEGMENT_SIZE);
                segment_2 = Square::new(segment_1.right(), segment_1.top(), SEGMENT_SIZE);
                segment_3 = Square::new(segment_2.left(), segment_2.top() - SEGMENT_SIZE, SEGMENT_SIZE); 
            },
            Shape::S => {
                segment_0 = Square::new(position.x - 0.5 * SEGMENT_SIZE, position.y - 1.5 * SEGMENT_SIZE, SEGMENT_SIZE);
                segment_1 = Square::new(segment_0.right(), segment_0.top(), SEGMENT_SIZE); 
                segment_2 = Square::new(segment_0.left() - SEGMENT_SIZE, segment_0.bottom(), SEGMENT_SIZE); 
                segment_3 = Square::new(segment_2.right(), segment_1.bottom(), SEGMENT_SIZE);
            },
            Shape::Z => {
                segment_0 = Square::new(position.x - 1.5 * SEGMENT_SIZE, position.y - 1.5 * SEGMENT_SIZE, SEGMENT_SIZE); 
                segment_1 = Square::new(segment_0.right(), segment_0.top(), SEGMENT_SIZE);
                segment_2 = Square::new(segment_1.left(), segment_1.bottom(), SEGMENT_SIZE);
                segment_3 = Square::new(segment_2.right(), segment_1.bottom(), SEGMENT_SIZE); 
            },
        }

        [segment_0, segment_1, segment_2, segment_3]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tetromino {
    position: Vec2, // center as shown in `SRS-pieces.webp`
    segments: [Square; 4],
    shape: Shape,
}

// constructors
impl Tetromino {
    pub fn new_i(position: Vec2) -> Tetromino {
        Tetromino {
            position,
            shape: Shape::I,
            segments: Shape::I.segments(position),
        }
    }
    pub fn new_o(position: Vec2) -> Tetromino {
        Tetromino {
            position,
            shape: Shape::O,
            segments: Shape::O.segments(position),
        }
    }
    pub fn new_t(position: Vec2) -> Tetromino {
        Tetromino {
            position,
            shape: Shape::T,
            segments: Shape::T.segments(position),
        }
    }
    pub fn new_j(position: Vec2) -> Tetromino {
        Tetromino {
            position,
            shape: Shape::J,
            segments: Shape::J.segments(position)
        }
    }
    pub fn new_l(position: Vec2) -> Tetromino {
        Tetromino { 
            position,
            shape: Shape::L,
            segments: Shape::L.segments(position),
        }
    }
    pub fn new_s(position: Vec2) -> Tetromino {
        Tetromino { 
            position,
            segments: Shape::S.segments(position),
            shape: Shape::S,
        }
    }
    pub fn new_z(position: Vec2) -> Tetromino {
        Tetromino {
            position,
            shape: Shape::Z,
            segments: Shape::Z.segments(position),
        }
    }
    pub fn random_at(position: Vec2) -> Tetromino {
        match rand() % 7 {
            0 => Tetromino::new_i(position),
            1 => Tetromino::new_o(position),
            2 => Tetromino::new_t(position),
            3 => Tetromino::new_j(position),
            4 => Tetromino::new_l(position),
            5 => Tetromino::new_s(position),
            6 => Tetromino::new_z(position),
            _ => Tetromino::new_i(position),
        }
    }
    pub fn random() -> Tetromino {
        Tetromino::random_at(Vec2::ZERO)
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
    pub fn position(&self) -> &Vec2 {
        &self.position
    }
    pub fn top(&self) -> f32 {
        let mut top_most = self.segments[0].top();
        for square in self.segments.iter() {
            if square.top() < top_most {
                top_most = square.right();
            }
        }
        top_most        
    }
    pub fn bottom(&self) -> f32 {
        let mut bottom_most = self.segments[0].bottom();
        for square in self.segments.iter() {
            if square.bottom() > bottom_most {
                bottom_most = square.bottom()
            }
        }
        bottom_most
    }
    pub fn left(&self) -> f32 {
        let mut left_most = self.segments[0].left();
        for square in self.segments.iter() {
            if square.left() < left_most {
                left_most = square.left()
            }
        }
        left_most
    }
    pub fn right(&self) -> f32 {
        let mut right_most = self.segments[0].right();
        for square in self.segments.iter() {
            if square.right() > right_most {
                right_most = square.right();
            }
        }
        right_most        
    }
}

// setters
impl Tetromino {
    pub fn add_position(&mut self, delta: Vec2) {
        for square in self.segments.iter_mut() {
            square.x += delta.x;
            square.y += delta.y;
        }
        self.position += delta;
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.segments = self.shape.segments(position); // resets orientation
    }
}

// actions
fn rotate_point(clockwise: bool, point: Vec2) -> Vec2 {
    if clockwise {
        vec2(point.y, -point.x)
    } else {
        vec2(-point.y, point.x)
    }
}
impl Tetromino {
    pub fn check_collision(&self, other: &Tetromino) -> bool {
        for self_seg in self.segments.iter() {
            for other_seg in other.segments.iter() {
                if self_seg == other_seg {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn move_left(&mut self) {
        self.add_position(-Vec2::X * SEGMENT_SIZE);
    }
    pub fn move_right(&mut self) {
        self.add_position(Vec2::X * SEGMENT_SIZE);
    }
    pub fn move_down(&mut self) {
        self.add_position(Vec2::Y * SEGMENT_SIZE);
    }
    pub fn move_up(&mut self) {
        self.add_position(-Vec2::Y * SEGMENT_SIZE);
    }
    pub fn rotate(&mut self, clockwise: bool) {
        for square in self.segments.iter_mut() {
            let segment_center = vec2( // square st
                square.x + 0.5 * SEGMENT_SIZE,
                square.y + 0.5 * SEGMENT_SIZE,
            );

            let rotated = rotate_point(clockwise, segment_center - self.position) + self.position;
            
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

    let i = Tetromino::new_i(origin);
    let o = Tetromino::new_o(origin);
    let t = Tetromino::new_t(origin);
    let j = Tetromino::new_j(origin);
    let l = Tetromino::new_l(origin);
    let s = Tetromino::new_s(origin);
    let z = Tetromino::new_z(origin);

    [i, o, t, j, l, s, z]
}