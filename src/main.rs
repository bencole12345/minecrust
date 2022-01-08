mod engine;
mod game;
mod world;

// TODO: Split into separate libraries like the Piston library uses

extern crate gl;
extern crate glfw;
extern crate glm;
extern crate image;
extern crate nalgebra as na;
extern crate packer;

fn main() {
    let mut driver = game::driver::Driver::new();
    driver.run_game();
}
