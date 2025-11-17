use macroquad::prelude::*;
use crate::particle::Particle;
use crate::particle_generator::ParticleGenerator;
use crate::controls::Controls;

/// Handle mouse input and spawn particles
pub fn handle_mouse_input(
    particles: &mut Vec<Particle>, 
    prev_mouse_pos: Vec2, 
    current_mouse_pos: Vec2,
    generator: &mut ParticleGenerator, 
    controls: &Controls
) {
    // Check if mouse is pressed
    if is_mouse_button_down(MouseButton::Left) {
        // Create multiple particles along the mouse path
        let distance = (current_mouse_pos - prev_mouse_pos).length();
        let steps = (distance / 5.0).ceil() as usize;
        
        for i in 0..steps {
            let t = if steps > 1 { i as f32 / (steps - 1) as f32 } else { 0.5 };
            let pos = prev_mouse_pos.lerp(current_mouse_pos, t);
            
            // Create a burst of particles at this position
            for _ in 0..5 {
                particles.push(generator.create_particle(pos.x, pos.y, controls.fountain_mode, controls.color_theme));
            }
        }
    }
}