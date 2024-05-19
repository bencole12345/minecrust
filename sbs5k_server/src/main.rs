mod backing_store;
mod player_state;
mod request_processor;

use std::io;

use backing_store::InMemoryBackingStore;

use crate::request_processor::RequestProcessor;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let backing_store = Box::<InMemoryBackingStore>::default();

    // TODO: Set up args here
    let addr = "127.0.0.1:12345";

    let mut request_processor = RequestProcessor::new(addr, backing_store).await;
    request_processor.main_loop().await?;

    Ok(())
}
