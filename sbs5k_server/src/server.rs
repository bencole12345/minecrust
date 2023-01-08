use tonic::{Request, Response, Status, Streaming};

use sbs5k_messages::{backend, events};

#[derive(Clone, Debug, Default)]
pub struct SBS5kServerImpl;

#[tonic::async_trait]
impl backend::Sbs5kBackend for SBS5kServerImpl {
    type StreamEventsStream = Streaming<events::Event>;

    async fn get_initial_state(
        &self,
        _request: Request<backend::GetInitialStateRequest>,
    ) -> Result<Response<backend::GetInitialStateResponse>, Status> {
        todo!()
    }

    async fn get_chunks(
        &self,
        _request: Request<backend::GetChunksRequest>,
    ) -> Result<Response<backend::GetChunksResponse>, Status> {
        todo!()
    }

    async fn stream_events(
        &self,
        _request: Request<Self::StreamEventsStream>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        todo!()
    }
}
