mod client;
mod engine;
mod world;

// TODO: Split into separate libraries like the Piston library uses

extern crate gl;
extern crate glfw;
extern crate glm;
extern crate image;
extern crate nalgebra as na;
extern crate packer;

fn main() {
    let chunks_source = Box::new(world::generation::FlatTerrainGenerator::default());
    let mut driver = client::Driver::new(chunks_source);
    driver.run_game();
}
