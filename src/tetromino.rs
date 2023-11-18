use std::ops::Deref;

use macroquad::prelude::*;

pub const SQUARE_SIZE: f32 = 20.0;
pub const BORDER_COLOR: Color = BLACK;
pub const BORDER_SIZE: f32 = 1.0;

pub struct Square(Rect);
impl Square {
    pub fn new(x: f32, y: f32, size: f32) -> Square {
        Square(
            Rect { x, y, w: size, h: size }
        )
    }
}
impl Deref for Square { // allows me to dereference any Square to get to the underlying Rect 
    type Target = Rect;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Shape::I => "I",
            Shape::O => "O",
            Shape::T => "T",
            Shape::J => "J",
            Shape::L => "L",
            Shape::S => "S",
            Shape::Z => "Z",
        })
    }
}

pub struct Tetromino {
    position: Vec2, // should this be the x and y of which ever Rect is top left?
    squares: [Square; 4],
    shape: Shape,
}
// constructors
impl Tetromino {
    pub fn new_I(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(0.0,              0.0, SQUARE_SIZE);
        let square_1 = Square::new(square_0.right(), 0.0, SQUARE_SIZE);
        let square_2 = Square::new(square_1.right(), 0.0, SQUARE_SIZE);
        let square_3 = Square::new(square_2.right(), 0.0, SQUARE_SIZE);

        Tetromino {
            shape: Shape::I,
            position: vec2(x, y),
            squares: [square_0, square_1, square_2, square_3],
        }
    }
    pub fn new_O(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(0.0,              0.0,               SQUARE_SIZE);
        let square_1 = Square::new(square_0.right(), 0.0,               SQUARE_SIZE);
        let square_2 = Square::new(square_0.left(),  square_0.bottom(), SQUARE_SIZE);
        let square_3 = Square::new(square_1.left(),  square_1.bottom(), SQUARE_SIZE);

        Tetromino {
            shape: Shape::O,
            position: vec2(x, y),
            squares: [square_0, square_1, square_2, square_3],
        }
    }
    pub fn new_T(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(0.0,              0.0, SQUARE_SIZE);
        let square_1 = Square::new(square_0.right(), 0.0, SQUARE_SIZE);
        let square_2 = Square::new(square_1.right(), 0.0, SQUARE_SIZE);
        let square_3 = Square::new(square_1.left(), square_1.bottom(), SQUARE_SIZE);

        Tetromino {
            shape: Shape::T,
            position: vec2(x, y),
            squares: [square_0, square_1, square_2, square_3],
        }
    }
    pub fn new_J(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(0.0,              0.0, SQUARE_SIZE);
        let square_1 = Square::new(square_0.left(), square_0.bottom(), SQUARE_SIZE);
        let square_2 = Square::new(square_1.left(), square_1.bottom(), SQUARE_SIZE);
        let square_3 = Square::new(square_2.left() - square_2.w, square_2.top(), SQUARE_SIZE);

        Tetromino {
            shape: Shape::J,
            position: vec2(x, y),
            squares: [square_0, square_1, square_2, square_3],
        }
    }
    pub fn new_L(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(0.0,              0.0, SQUARE_SIZE);
        let square_1 = Square::new(square_0.left(), square_0.bottom(), SQUARE_SIZE);
        let square_2 = Square::new(square_1.left(), square_1.bottom(), SQUARE_SIZE);
        let square_3 = Square::new(square_2.right(), square_2.top(), SQUARE_SIZE);

        Tetromino { 
            position: vec2(x, y),
            squares: [square_0, square_1, square_2, square_3],
            shape: Shape::L
        }
    }
    pub fn new_S(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(0.0, 0.0, SQUARE_SIZE);
        let square_1 = Square::new(square_0.right(), 0.0, SQUARE_SIZE);
        let square_2 = Square::new(square_0.left() - square_0.w, square_0.bottom(), SQUARE_SIZE);
        let square_3 = Square::new(square_2.right(), square_1.bottom(), SQUARE_SIZE);

        Tetromino { 
            position: vec2(x, y),
            squares: [square_0, square_1, square_2, square_3],
            shape: Shape::S,
        }
    }
    pub fn new_Z(x: f32, y: f32) -> Tetromino {
        let square_0 = Square::new(0.0, 0.0, SQUARE_SIZE);
        let square_1 = Square::new(square_0.right(), 0.0, SQUARE_SIZE);
        let square_2 = Square::new(square_1.left(), square_1.bottom(), SQUARE_SIZE);
        let square_3 = Square::new(square_2.right(), square_1.bottom(), SQUARE_SIZE);


        Tetromino {
            shape: Shape::Z,
            position: vec2(x, y),
            squares: [square_0, square_1, square_2, square_3],
        }
    }
}

// getters
impl Tetromino {
    pub fn shape(&self) -> Shape {
        self.shape
    }
}

impl Tetromino {
    pub fn draw(&self) {
        let interior_color = match self.shape {
            Shape::I => BLUE,
            Shape::O => YELLOW,
            Shape::T => MAGENTA,
            Shape::J => PURPLE,
            Shape::L => ORANGE,
            Shape::S => GREEN,
            Shape::Z => RED,
        };

        for sq in self.squares.iter() {
            // draw border
            draw_rectangle(
                sq.x + self.position.x,
                sq.y + self.position.y,
                sq.w,
                sq.h,
                BORDER_COLOR
            );

            // draw interior
            draw_rectangle(
                sq.x + self.position.x + BORDER_SIZE,
                sq.y + self.position.y + BORDER_SIZE,
                sq.w - 2.0 * BORDER_SIZE,
                sq.h - 2.0 * BORDER_SIZE,
                interior_color
            );
        }
    }
}

pub fn all_shapes() -> [Tetromino; 7] {
    let origin = vec2(screen_width()/2.0, screen_height()/2.0);

    let i = Tetromino::new_I(origin.x, origin.y);
    let o = Tetromino::new_O(origin.x - 100.0, origin.y);
    let t = Tetromino::new_T(origin.x + 100.0, origin.y);
    let j = Tetromino::new_J(origin.x, origin.y - 100.0);
    let l = Tetromino::new_L(origin.x, origin.y + 100.0);
    let s = Tetromino::new_S(origin.x + 100.0, origin.y + 100.0);
    let z = Tetromino::new_Z(origin.x - 100.0, origin.y - 100.0);

    [i, o, t, j, l, s, z]
}