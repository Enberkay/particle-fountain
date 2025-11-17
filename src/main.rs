mod particle;
mod physics;
mod ui;
mod input;
mod game;

use game::run_game_loop;

#[macroquad::main("Particle Fountain")]
async fn main() {
    run_game_loop().await
}