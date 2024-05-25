mod backing_store;
mod client_task;
mod server;
mod state;

use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;

use backing_store::InMemoryBackingStore;

use crate::server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let backing_store = Arc::new(Mutex::new(InMemoryBackingStore::default()));

    // TODO: Set up args here
    let addr = "127.0.0.1:12345";

    let mut request_processor = Server::new(addr, backing_store).await;
    request_processor.main_loop().await?;

    Ok(())
}
