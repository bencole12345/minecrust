mod request_processor;

use std::io;

use crate::request_processor::RequestProcessor;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:12345";

    let request_processor = RequestProcessor::new(addr).await;
    request_processor.main_loop().await?;

    Ok(())
}
