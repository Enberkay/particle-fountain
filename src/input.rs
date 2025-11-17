use macroquad::prelude::*;
use crate::particle::Particle;

/// Handle mouse input and spawn particles
pub fn handle_input(particles: &mut Vec<Particle>, prev_mouse_pos: Vec2, mouse_pos: Vec2) {
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
}