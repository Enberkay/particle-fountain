use macroquad::prelude::*;
use crate::particle_generator::{FountainMode, ColorTheme};
use crate::effects::EffectMode;

/// Draw UI text showing particle count and instructions
pub fn draw_ui(particle_count: usize, fountain_mode: FountainMode, color_theme: ColorTheme, gravity_multiplier: f32, effect_mode: EffectMode) {
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
        &format!("Mode: {:?} | Theme: {:?} | Gravity: {:.1}x | Effect: {:?}", 
               fountain_mode, color_theme, gravity_multiplier, effect_mode),
        10.0,
        50.0,
        16.0,
        WHITE,
    );
    
    // Draw instructions
    draw_text(
        "Click and drag to create fountain | 1-5: Mode | R: Theme | G: Gravity | E: Effect | Space: Clear",
        10.0,
        screen_height() - 20.0,
        16.0,
        WHITE,
    );
}