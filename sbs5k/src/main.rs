mod args;
mod constants;
mod controls;
mod debug;
mod driver;
mod initialisation;
mod loading;
mod resources;
mod state;

use clap::Parser;

use sbs5k_core::generators::PerlinNoiseGenerator;

use crate::args::Args;
use crate::driver::Driver;

fn main() {
    let config = Args::parse();

    let chunks_source = Box::new(PerlinNoiseGenerator::new());
    let mut driver = Driver::new(config);
    driver.run_game(chunks_source);
}
