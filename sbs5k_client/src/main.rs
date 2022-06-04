mod constants;
mod controls;
mod debug;
mod driver;
mod initialisation;
mod loading;
mod resources;
mod state;

use sbs5k_world::generators::PerlinNoiseGenerator;

use crate::driver::Driver;

fn main() {
    let chunks_source = Box::new(PerlinNoiseGenerator::new());
    let mut driver = Driver::new();
    driver.run_game(chunks_source);
}
