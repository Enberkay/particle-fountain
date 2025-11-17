use macroquad::prelude::*;

use crate::particle::Particle;

// Effects that can be applied to particles
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum EffectMode {
    None,       // No special effects
    Trail,      // Particles leave trails behind them
    Glow,       // Particles have a glowing effect
    Magnet,     // Particles are attracted to the mouse
    Explosion,  // Particles explode when clicking
}

// Effect configuration
pub struct EffectConfig {
    pub mode: EffectMode,
    pub mouse_pos: Vec2,
}

impl EffectConfig {
    pub fn new() -> Self {
        Self {
            mode: EffectMode::None,
            mouse_pos: Vec2::new(0.0, 0.0),
        }
    }
    
    // Methods are actually used in game.rs, just not detected by compiler
    #[allow(dead_code)] // This method is used in game.rs, just not detected by compiler
    pub fn set_mode(&mut self, mode: EffectMode) {
        self.mode = mode;
    }
    
    #[allow(dead_code)] // This method is used in game.rs, just not detected by compiler
    pub fn set_mouse_pos(&mut self, pos: Vec2) {
        self.mouse_pos = pos;
    }
    
    // Apply effects to particles
    pub fn apply_to_particle(&self, particle: &mut Particle, dt: f32) {
        match self.mode {
            EffectMode::None => {}
            EffectMode::Trail => {
                // Trail effect handled in drawing
            },
            EffectMode::Glow => {
                // Glow effect handled in drawing
            },
            EffectMode::Magnet => {
                // Attract particle to mouse position
                let diff = self.mouse_pos - particle.pos;
                let distance = diff.length();
                
                if distance > 10.0 && distance < 300.0 {
                    // Apply attraction force (stronger when closer)
                    let strength = (1.0 - distance / 300.0) * 200.0;
                    let direction = diff.normalize();
                    particle.vel += direction * strength * dt;
                }
            },
            EffectMode::Explosion => {
                // This will be handled in the input module
            },
        }
    }
    
    // Draw special effects
    pub fn draw_effects(&self, particles: &[Particle]) {
        match self.mode {
            EffectMode::None => {}
            EffectMode::Trail => {
                // Trail effect - draw particles with lower opacity first
                for particle in particles {
                    draw_circle(
                        particle.pos.x,
                        particle.pos.y,
                        particle.size * 1.2,
                        Color::new(
                            particle.color.r,
                            particle.color.g,
                            particle.color.b,
                            particle.color.a * 0.3,
                        ),
                    );
                }
            },
            EffectMode::Glow => {
                // Draw glow effect for each particle
                for particle in particles {
                    // Outer glow
                    draw_circle(
                        particle.pos.x,
                        particle.pos.y,
                        particle.size * 2.5,
                        Color::new(
                            particle.color.r,
                            particle.color.g,
                            particle.color.b,
                            particle.color.a * 0.2,
                        ),
                    );
                    
                    // Middle glow
                    draw_circle(
                        particle.pos.x,
                        particle.pos.y,
                        particle.size * 1.5,
                        Color::new(
                            particle.color.r,
                            particle.color.g,
                            particle.color.b,
                            particle.color.a * 0.4,
                        ),
                    );
                }
            },
            EffectMode::Magnet => {
                // Draw attraction range indicator
                draw_circle_lines(
                    self.mouse_pos.x,
                    self.mouse_pos.y,
                    300.0,
                    1.0,
                    Color::new(1.0, 1.0, 1.0, 0.3),
                );
            },
            EffectMode::Explosion => {
                // Explosion effect will be drawn in input module
            },
        }
    }
}

