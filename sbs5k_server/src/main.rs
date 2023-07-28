mod args;
mod server;

use clap::Parser;
use tokio;
use tokio::io;

use crate::args::Args;
use crate::server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = Args::parse();
    let server = Server::new(config);

    server.main_loop().await?;

    Ok(())
}
