mod particle;
mod physics;
use macroquad::prelude::*;
use particle::Particle;

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
        
        // Check if mouse is pressed
        if is_mouse_button_down(MouseButton::Left) {
            // Create multiple particles along the mouse path
            let distance = (mouse_pos - prev_mouse_pos).length();
            let steps = (distance / 5.0).ceil() as usize;
            
            for i in 0..steps {
                let t = if steps > 1 { i as f32 / (steps - 1) as f32 } else { 0.5 };
                let pos = prev_mouse_pos.lerp(mouse_pos, t);
                
                // Create a burst of particles at this position
                for _ in 0..5 {
                    particles.push(Particle::new(pos.x, pos.y));
                }
            }
        }
        
        // Check if mouse is pressed
        if is_mouse_button_down(MouseButton::Left) {
            // Create multiple particles along the mouse path
            let distance = (mouse_pos - prev_mouse_pos).length();
            let steps = (distance / 5.0).ceil() as usize;
            
            for i in 0..steps {
                let t = if steps > 1 { i as f32 / (steps - 1) as f32 } else { 0.5 };
                let pos = prev_mouse_pos.lerp(mouse_pos, t);
                
                // Create a burst of particles at this position
                for _ in 0..5 {
                    particles.push(Particle::new(pos.x, pos.y));
                }
            }
        }
        
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
        draw_text(
            &format!("Particles: {}", particles.len()),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        
        draw_text(
            "Click and drag to create a fountain of particles!",
            10.0,
            screen_height() - 20.0,
            20.0,
            WHITE,
        );
        
        next_frame().await
    }
}