use clap::Parser;
use std::net;

#[derive(Clone, Copy, Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(short, long, default_value_t = 10)]
    /// The radius of blocks around the player to render
    pub render_distance: u32,

    #[clap(short, long)]
    /// Print additional information to the console
    pub verbose: bool,

    #[clap(long)]
    /// Print the framerate to the console every frame
    pub debug_print_fps: bool,

    #[clap(long)]
    /// Print the player's position to the console every frame
    pub debug_print_player_position: bool,

    #[clap(long)]
    /// The address of the server
    pub server: Option<net::SocketAddr>,
}

impl Args {
    #[inline(always)]
    pub(crate) fn is_in_debug_mode(&self) -> bool {
        self.debug_print_fps || self.debug_print_player_position
    }
}
