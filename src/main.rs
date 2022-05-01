mod client;
mod engine;
mod utils;
mod world;

// TODO: Split into separate libraries like the Piston library uses

extern crate gl;
extern crate glfw;
extern crate glm;
extern crate image;
extern crate nalgebra as na;
extern crate packer;
extern crate rand;

#[cfg(test)]
extern crate rstest;

fn main() {
    let chunks_source = Box::new(world::generators::PerlinNoiseGenerator::new());
    let mut driver = client::Driver::new();
    driver.run_game(chunks_source);
}
