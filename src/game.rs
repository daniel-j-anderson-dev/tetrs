use crate::tetromino::Tetromino;
pub struct Game {
    score: u32,
    is_over: bool,
    current_piece: Tetromino,
    held_piece: Option<Tetromino>,
    next_piece: Tetromino,
}
impl std::default::Default for Game {
    fn default() -> Self {
        Self {
            score: 0,
            is_over: false,
            current_piece: Tetromino::random_at_origin(),
            held_piece: None,
            next_piece: Tetromino::random_at_origin(),
        }
    }
}