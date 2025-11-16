use macroquad::prelude::*;
use macroquad::rand::gen_range;

// Particle structure to represent each particle in our fountain
#[derive(Debug)]
struct Particle {
    // Position (x, y)
    pos: Vec2,
    // Velocity (x, y)
    vel: Vec2,
    // Color of the particle
    color: Color,
    // How long the particle will live (in seconds)
    lifetime: f32,
    // Maximum lifetime for this particle
    max_lifetime: f32,
    // Size of the particle
    size: f32,
}

#[macroquad::main("Particle Fountain")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}