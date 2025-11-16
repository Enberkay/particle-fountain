use macroquad::prelude::*;

#[macroquad::main("Particle Fountain")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}