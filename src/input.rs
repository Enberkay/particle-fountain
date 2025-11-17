use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::particle::Particle;
use crate::particle_generator::ParticleGenerator;
use crate::controls::Controls;
use crate::effects::EffectMode;

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
    
    // Handle explosion effect when mouse button is released
    if controls.effect_mode == EffectMode::Explosion && 
       is_mouse_button_released(MouseButton::Left) {
        create_explosion(current_mouse_pos, particles);
    }
}

// Create explosion effect at position
fn create_explosion(pos: Vec2, particles: &mut Vec<Particle>) {
    // Create explosion particles
    for _ in 0..30 {
        let angle = gen_range(0.0, 2.0 * std::f32::consts::PI);
        let speed = gen_range(50.0, 300.0);
        let vel = Vec2::new(
            angle.cos() * speed,
            angle.sin() * speed,
        );
        
        // Create explosion particle with orange/red colors
        let color = if gen_range(0.0, 1.0) > 0.5 {
            Color::new(
                gen_range(0.8, 1.0), // Red
                gen_range(0.2, 0.5), // Green
                gen_range(0.0, 0.1), // Blue
                1.0,
            )
        } else {
            Color::new(
                gen_range(0.8, 1.0), // Red
                gen_range(0.4, 0.8), // Green
                0.0,               // Blue
                1.0,
            )
        };
        
        let lifetime = gen_range(0.5, 1.5);
        let size = gen_range(5.0, 15.0);
        
        particles.push(Particle {
            pos,
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size,
        });
    }
}

