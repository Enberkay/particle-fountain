use macroquad::prelude::*;

use crate::physics::{apply_gravity, apply_air_resistance, update_position};

// Particle structure to represent each particle in our fountain
#[derive(Debug)]
pub struct Particle {
    // Position (x, y)
    pub pos: Vec2,
    // Velocity (x, y)
    pub vel: Vec2,
    // Color of the particle
    pub color: Color,
    // How long the particle will live (in seconds)
    pub lifetime: f32,
    // Maximum lifetime for this particle
    pub max_lifetime: f32,
    // Size of the particle
    pub size: f32,
}

impl Particle {
    
    // Update with custom gravity
    pub fn update_with_gravity(&mut self, dt: f32, gravity: f32) {
        // Apply physics
        apply_gravity(&mut self.vel, dt, gravity);
        apply_air_resistance(&mut self.vel);
        update_position(&mut self.pos, &self.vel, dt);
        
        // Decrease lifetime
        self.lifetime -= dt;
    }
    
    // Check if the particle is still alive
    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
    
    // Draw the particle
    pub fn draw(&self) {
        // Calculate opacity based on remaining lifetime
        let opacity = self.lifetime / self.max_lifetime;
        let color = Color::new(
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a * opacity,
        );
        
        draw_circle(self.pos.x, self.pos.y, self.size, color);
    }
}