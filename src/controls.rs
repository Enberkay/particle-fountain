use macroquad::prelude::*;
use crate::particle_generator::{FountainMode, ColorTheme};
use crate::effects::EffectMode;

// Control state
pub struct Controls {
    pub fountain_mode: FountainMode,
    pub color_theme: ColorTheme,
    pub gravity_multiplier: f32,
    pub clear_particles: bool,
    pub effect_mode: EffectMode,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            fountain_mode: FountainMode::Normal,
            color_theme: ColorTheme::Random,
            gravity_multiplier: 1.0,
            clear_particles: false,
            effect_mode: EffectMode::None,
        }
    }
    
    // Handle keyboard input
    pub fn update(&mut self) {
        // Reset clear flag
        self.clear_particles = false;
        
        // Number keys for fountain modes
        if is_key_pressed(KeyCode::Key1) {
            self.fountain_mode = FountainMode::Normal;
        } else if is_key_pressed(KeyCode::Key2) {
            self.fountain_mode = FountainMode::Wide;
        } else if is_key_pressed(KeyCode::Key3) {
            self.fountain_mode = FountainMode::Narrow;
        } else if is_key_pressed(KeyCode::Key4) {
            self.fountain_mode = FountainMode::Explode;
        } else if is_key_pressed(KeyCode::Key5) {
            self.fountain_mode = FountainMode::Spiral;
        }
        
        // R for color modes
        if is_key_pressed(KeyCode::R) {
            self.color_theme = match self.color_theme {
                ColorTheme::Random => ColorTheme::Rainbow,
                ColorTheme::Rainbow => ColorTheme::Fire,
                ColorTheme::Fire => ColorTheme::Water,
                ColorTheme::Water => ColorTheme::Nature,
                ColorTheme::Nature => ColorTheme::Random,
            };
        }
        
        // G for gravity adjustment
        if is_key_pressed(KeyCode::G) {
            self.gravity_multiplier = match self.gravity_multiplier {
                0.5 => 1.0,
                1.0 => 1.5,
                1.5 => 2.0,
                2.0 => 0.5,
                _ => 1.0,
            };
        }
        
        // Space to clear particles
        if is_key_pressed(KeyCode::Space) {
            self.clear_particles = true;
        }
    }
}