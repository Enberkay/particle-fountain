use macroquad::prelude::*;
use crate::particle_generator::{FountainMode, ColorTheme};

/// Draw UI text showing particle count and instructions
pub fn draw_ui(particle_count: usize, fountain_mode: FountainMode, color_theme: ColorTheme, gravity_multiplier: f32) {
    // Draw particle count
    draw_text(
        &format!("Particles: {}", particle_count),
        10.0,
        20.0,
        20.0,
        WHITE,
    );
    
    // Draw control information
    draw_text(
        &format!("Mode: {:?} | Theme: {:?} | Gravity: {:.1}x", 
               fountain_mode, color_theme, gravity_multiplier),
        10.0,
        50.0,
        16.0,
        WHITE,
    );
    
    // Draw instructions
    draw_text(
        "Click and drag to create fountain | 1-5: Mode | R: Theme | G: Gravity | Space: Clear",
        10.0,
        screen_height() - 20.0,
        16.0,
        WHITE,
    );
}