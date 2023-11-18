use macroquad::prelude::*;

#[macroquad::main("tetrs")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        clear_background(LIGHTGRAY);

        next_frame().await;
    }
}
