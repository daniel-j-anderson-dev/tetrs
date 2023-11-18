mod tetromino;

use macroquad::prelude::*;
use tetromino::Tetromino;

#[macroquad::main("tetrs")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pieces: [Tetromino; 7] = tetromino::all_shapes();

    loop {
        clear_background(LIGHTGRAY);

        for piece in &pieces {
            piece.draw();
        }

        next_frame().await;
    }
}
