use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::particle::Particle;

// Fountain modes
#[derive(Debug, Clone, Copy)]
pub enum FountainMode {
    Normal,
    Wide,
    Narrow,
    Explode,
    Spiral,
}

// Color themes
#[derive(Debug, Clone, Copy)]
pub enum ColorTheme {
    Random,
    Rainbow,
    Fire,
    Water,
    Nature,
}

pub struct ParticleGenerator {
    pub particle_counter: usize,
}

impl ParticleGenerator {
    pub fn new() -> Self {
        Self {
            particle_counter: 0,
        }
    }
    
    // Generate particle color based on theme
    fn get_particle_color(&self, theme: ColorTheme) -> Color {
        match theme {
            ColorTheme::Random => {
                Color::new(
                    gen_range(0.5, 1.0),
                    gen_range(0.5, 1.0),
                    gen_range(0.5, 1.0),
                    1.0,
                )
            },
            ColorTheme::Rainbow => {
                let hue = (self.particle_counter as f32 * 0.1) % 1.0;
                let rgb = hsl_to_rgb(hue, 0.8, 0.7);
                Color::new(rgb.0, rgb.1, rgb.2, 1.0)
            },
            ColorTheme::Fire => {
                let r = gen_range(0.7, 1.0);
                let g = gen_range(0.2, 0.7);
                let b = gen_range(0.0, 0.2);
                Color::new(r, g, b, 1.0)
            },
            ColorTheme::Water => {
                let r = gen_range(0.0, 0.3);
                let g = gen_range(0.3, 0.7);
                let b = gen_range(0.7, 1.0);
                Color::new(r, g, b, 1.0)
            },
            ColorTheme::Nature => {
                let r = gen_range(0.1, 0.5);
                let g = gen_range(0.3, 0.8);
                let b = gen_range(0.1, 0.3);
                Color::new(r, g, b, 1.0)
            },
        }
    }
    
    // Generate particle velocity based on fountain mode
    fn get_particle_velocity(&self, mode: FountainMode) -> Vec2 {
        match mode {
            FountainMode::Normal => {
                let angle = gen_range(-std::f32::consts::PI / 4.0, std::f32::consts::PI / 4.0) - std::f32::consts::PI / 2.0;
                let speed = gen_range(200.0, 400.0);
                Vec2::new(angle.cos() * speed, angle.sin() * speed)
            },
            FountainMode::Wide => {
                let angle = gen_range(-std::f32::consts::PI / 2.5, std::f32::consts::PI / 2.5) - std::f32::consts::PI / 2.0;
                let speed = gen_range(150.0, 350.0);
                Vec2::new(angle.cos() * speed, angle.sin() * speed)
            },
            FountainMode::Narrow => {
                let angle = gen_range(-std::f32::consts::PI / 8.0, std::f32::consts::PI / 8.0) - std::f32::consts::PI / 2.0;
                let speed = gen_range(300.0, 500.0);
                Vec2::new(angle.cos() * speed, angle.sin() * speed)
            },
            FountainMode::Explode => {
                let angle = gen_range(0.0, 2.0 * std::f32::consts::PI);
                let speed = gen_range(100.0, 600.0);
                Vec2::new(angle.cos() * speed, angle.sin() * speed)
            },
            FountainMode::Spiral => {
                let base_angle = (self.particle_counter as f32 * 0.5) % (2.0 * std::f32::consts::PI);
                let angle = base_angle - std::f32::consts::PI / 2.0;
                let speed = gen_range(250.0, 450.0);
                Vec2::new(angle.cos() * speed, angle.sin() * speed)
            },
        }
    }
    
    // Create a new particle with the specified mode and theme
    pub fn create_particle(&mut self, x: f32, y: f32, mode: FountainMode, theme: ColorTheme) -> Particle {
        let color = self.get_particle_color(theme);
        let vel = self.get_particle_velocity(mode);
        
        // Random lifetime between 1 and 3 seconds
        let lifetime = gen_range(1.0, 3.0);
        
        // Random size
        let size = gen_range(3.0, 8.0);
        
        self.particle_counter += 1;
        
        Particle {
            pos: Vec2::new(x, y),
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size,
        }
    }
    
    // Reset particle counter
    pub fn reset(&mut self) {
        self.particle_counter = 0;
    }
}

// HSL to RGB conversion helper
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h * 6.0;
    let x = c * (1.0 - ((hp % 2.0) - 1.0).abs());
    let m = l - c / 2.0;
    
    let (r, g, b) = if hp < 1.0 {
        (c, x, 0.0)
    } else if hp < 2.0 {
        (x, c, 0.0)
    } else if hp < 3.0 {
        (0.0, c, x)
    } else if hp < 4.0 {
        (0.0, x, c)
    } else if hp < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    (r + m, g + m, b + m)
}