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

impl Particle {
    // Create a new particle at the given position with random properties
    fn new(x: f32, y: f32) -> Self {
        // Random color - vibrant colors for the fountain
        let color = Color::new(
            gen_range(0.5, 1.0), // R
            gen_range(0.5, 1.0), // G
            gen_range(0.5, 1.0), // B
            1.0,                 // A (fully opaque initially)
        );
        
        // Random velocity pointing upward with some spread
        let angle = gen_range(-std::f32::consts::PI / 4.0, std::f32::consts::PI / 4.0) - std::f32::consts::PI / 2.0;
        let speed = gen_range(200.0, 400.0);
        let vel = Vec2::new(
            angle.cos() * speed,
            angle.sin() * speed,
        );
        
        // Random lifetime between 1 and 3 seconds
        let lifetime = gen_range(1.0, 3.0);
        
        // Random size
        let size = gen_range(3.0, 8.0);
        
        Self {
            pos: Vec2::new(x, y),
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size,
        }
    }
}

#[macroquad::main("Particle Fountain")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}