mod args;
mod backend_connection;
mod constants;
mod controls;
mod debug;
mod driver;
mod event;
mod initialisation;
mod loading;
mod networking;
mod resources;
mod state;
mod updatable;

use std::sync::Arc;

use clap::Parser;

use sbs5k_core::generators::PerlinNoiseGenerator;

use crate::args::Args;
use crate::driver::Driver;

fn main() {
    let config = Arc::new(Args::parse());

    let chunks_source = Box::new(PerlinNoiseGenerator::new());
    let mut driver = Driver::new(config, chunks_source);
    driver.run_game();
}
