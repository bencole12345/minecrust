mod server;

use tokio;
use tonic::transport::Server;

use sbs5k_messages::backend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let svc_instance = server::SBS5kServerImpl::default();

    println!("Running service on address {:?}", addr);

    Server::builder()
        .add_service(backend::Sbs5kBackendServer::new(svc_instance))
        .serve(addr)
        .await?;

    Ok(())
}
