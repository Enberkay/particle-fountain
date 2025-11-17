use macroquad::math::Vec2;

// Physics constants
pub const GRAVITY: f32 = 500.0;
pub const AIR_RESISTANCE: f32 = 0.99;

/// Apply gravity to a velocity vector
pub fn apply_gravity(vel: &mut Vec2, dt: f32) {
    vel.y += GRAVITY * dt;
}

/// Apply air resistance to a velocity vector
pub fn apply_air_resistance(vel: &mut Vec2) {
    vel.x *= AIR_RESISTANCE;
    vel.y *= AIR_RESISTANCE;
}

/// Update particle position based on velocity and delta time
pub fn update_position(pos: &mut Vec2, vel: &Vec2, dt: f32) {
    pos.x += vel.x * dt;
    pos.y += vel.y * dt;
}