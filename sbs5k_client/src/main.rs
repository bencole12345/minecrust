mod constants;
mod controls;
mod debug;
mod driver;
mod initialisation;
mod mesh_generation;
mod resources;
mod state;

use sbs5k_world::generators::PerlinNoiseGenerator;

use crate::driver::Driver;

fn main() {
    let chunks_source = Box::new(PerlinNoiseGenerator::new());
    let mut driver = Driver::new(chunks_source);
    driver.run_game();
}
