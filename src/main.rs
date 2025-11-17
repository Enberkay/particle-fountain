mod particle;
mod physics;
mod ui;
mod input;
use macroquad::prelude::*;
use particle::Particle;
use ui::draw_ui;
use input::handle_input;

#[macroquad::main("Particle Fountain")]
async fn main() {
    // Vector to store all particles
    let mut particles: Vec<Particle> = Vec::new();
    
    // Mouse position tracking
    let mut mouse_pos: Vec2 = mouse_position().into();
    
    loop {
        // Get delta time (time since last frame)
        let dt = get_frame_time();
        
        // Handle input
        let prev_mouse_pos = mouse_pos;
        mouse_pos = mouse_position().into();
        
        handle_input(&mut particles, prev_mouse_pos, mouse_pos);
        
        // Update all particles
        for particle in &mut particles {
            particle.update(dt);
        }
        
        // Remove dead particles
        particles.retain(|p| p.is_alive());
        
        // Drawing
        clear_background(BLACK);
        
        // Draw all particles
        for particle in &particles {
            particle.draw();
        }
        
        // Draw UI
        draw_ui(particles.len());
        
        next_frame().await
    }
}