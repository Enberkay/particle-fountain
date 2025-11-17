use macroquad::prelude::*;

/// Draw the UI text showing particle count and instructions
pub fn draw_ui(particle_count: usize) {
    draw_text(
        &format!("Particles: {}", particle_count),
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
}