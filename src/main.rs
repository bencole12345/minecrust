mod engine;
mod game;
mod world;

extern crate gl;
extern crate glfw;
extern crate glm;
extern crate nalgebra as na;

fn main() {
    game::bencraft::run_game();
}
