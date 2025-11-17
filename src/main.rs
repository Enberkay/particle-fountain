mod particle;
mod physics;
mod ui;
mod input;
mod game;
mod controls;
mod particle_generator;
mod effects;

use game::run_game_loop;

#[macroquad::main("Particle Fountain")]
async fn main() {
    run_game_loop().await
}