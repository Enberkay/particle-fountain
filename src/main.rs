use macroquad::prelude::*;
use macroquad::rand::gen_range;

// Particle structure to represent each particle in our fountain
#[derive(Debug)]
struct Particle {
    // Position (x, y)
    pos: Vec2,
    // Velocity (x, y)
    vel: Vec2,
    // Color of the particle
    color: Color,
    // How long the particle will live (in seconds)
    lifetime: f32,
    // Maximum lifetime for this particle
    max_lifetime: f32,
    // Size of the particle
    size: f32,
}

impl Particle {
    // Create a new particle at the given position with random properties
    fn new(x: f32, y: f32) -> Self {
        // Random color - vibrant colors for the fountain
        let color = Color::new(
            gen_range(0.5, 1.0), // R
            gen_range(0.5, 1.0), // G
            gen_range(0.5, 1.0), // B
            1.0,                 // A (fully opaque initially)
        );
        
        // Random velocity pointing upward with some spread
        let angle = gen_range(-std::f32::consts::PI / 4.0, std::f32::consts::PI / 4.0) - std::f32::consts::PI / 2.0;
        let speed = gen_range(200.0, 400.0);
        let vel = Vec2::new(
            angle.cos() * speed,
            angle.sin() * speed,
        );
        
        // Random lifetime between 1 and 3 seconds
        let lifetime = gen_range(1.0, 3.0);
        
        // Random size
        let size = gen_range(3.0, 8.0);
        
        Self {
            pos: Vec2::new(x, y),
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size,
        }
    }
    
    // Update the particle position and lifetime
    fn update(&mut self, dt: f32) {
        // Apply gravity to velocity (downward acceleration)
        self.vel.y += 500.0 * dt;
        
        // Apply slight air resistance to make it look more natural
        self.vel.x *= 0.99;
        self.vel.y *= 0.99;
        
        // Update position based on velocity
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
        
        // Decrease lifetime
        self.lifetime -= dt;
    }
    
    // Check if the particle is still alive
    fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
    
    // Draw the particle
    fn draw(&self) {
        // Calculate opacity based on remaining lifetime
        let opacity = self.lifetime / self.max_lifetime;
        let color = Color::new(
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a * opacity,
        );
        
        draw_circle(self.pos.x, self.pos.y, self.size, color);
    }
}

#[macroquad::main("Particle Fountain")]
async fn main() {
    // Vector to store all particles
    let mut particles: Vec<Particle> = Vec::new();
    
    // Mouse position tracking
    let mut mouse_pos: Vec2 = mouse_position().into();
    
    loop {
        // Get delta time (time since last frame)
        let dt = get_frame_time();
        
        // Handle input
        let prev_mouse_pos = mouse_pos;
        mouse_pos = mouse_position().into();
        
        // Check if mouse is pressed
        if is_mouse_button_down(MouseButton::Left) {
            // Create multiple particles along the mouse path
            let distance = (mouse_pos - prev_mouse_pos).length();
            let steps = (distance / 5.0).ceil() as usize;
            
            for i in 0..steps {
                let t = if steps > 1 { i as f32 / (steps - 1) as f32 } else { 0.5 };
                let pos = prev_mouse_pos.lerp(mouse_pos, t);
                
                // Create a burst of particles at this position
                for _ in 0..5 {
                    particles.push(Particle::new(pos.x, pos.y));
                }
            }
        }
        
        // Update all particles
        for particle in &mut particles {
            particle.update(dt);
        }
        
        // Remove dead particles
        particles.retain(|p| p.is_alive());
        
        // Drawing
        clear_background(BLACK);
        
        // Draw all particles
        for particle in &particles {
            particle.draw();
        }
        
        next_frame().await
    }
}