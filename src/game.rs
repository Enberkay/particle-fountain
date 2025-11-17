use macroquad::prelude::*;
use crate::particle::Particle;
use crate::ui::draw_ui;
use crate::input::handle_mouse_input;
use crate::controls::Controls;
use crate::particle_generator::ParticleGenerator;
use crate::physics::GRAVITY;
use crate::effects::EffectConfig;

/// Run the main game loop
pub async fn run_game_loop() {
    // Vector to store all particles
    let mut particles: Vec<Particle> = Vec::new();
    
    // Particle generator
    let mut generator = ParticleGenerator::new();
    
    // Controls
    let mut controls = Controls::new();
    
    // Effect configuration
    let mut effect_config = EffectConfig::new();
    
    // Mouse position tracking
    let mut mouse_pos: Vec2 = mouse_position().into();
    
    loop {
        // Get delta time (time since last frame)
        let dt = get_frame_time();
        
        // Handle keyboard input
        controls.update();
        
        // Handle input
        let prev_mouse_pos = mouse_pos;
        mouse_pos = mouse_position().into();
        
        handle_mouse_input(&mut particles, prev_mouse_pos, mouse_pos, &mut generator, &controls);
        
        // Check if we need to clear particles
        if controls.clear_particles {
            particles.clear();
            generator.reset();
        }
        
        // Apply effects to particles
        for particle in &mut particles {
            effect_config.apply_to_particle(particle, dt);
            particle.update_with_gravity(dt, GRAVITY * controls.gravity_multiplier);
        }
        
        // Update all particles with custom gravity
        for particle in &mut particles {
            particle.update_with_gravity(dt, GRAVITY * controls.gravity_multiplier);
        }
        

        
        // Remove dead particles
        particles.retain(|p| p.is_alive());
        
        // Drawing
        clear_background(BLACK);
        
        // Draw all particles
        for particle in &particles {
            particle.draw();
        }
        
        // Draw special effects
        effect_config.draw_effects(&particles);
        
        // Draw all particles
        for particle in &particles {
            particle.draw();
        }
        
        // Draw UI
        draw_ui(particles.len(), controls.fountain_mode, controls.color_theme, controls.gravity_multiplier, controls.effect_mode);
        
        next_frame().await
    }
}