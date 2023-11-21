use macroquad::math::Rect;

#[derive(Debug, PartialEq)]
pub struct Square(Rect);
impl Square {
    pub fn new(x: f32, y: f32, size: f32) -> Square {
        Square(
            Rect { x, y, w: size, h: size }
        )
    }
}
impl std::ops::Deref for Square { // allows me to dereference any Square to get to the underlying Rect 
    type Target = Rect;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Square {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}