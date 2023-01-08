/// Requests and responses for communicating with the backend service
pub mod backend {
    tonic::include_proto!("sbs5k_messages.backend");

    pub use sbs5k_backend_client::Sbs5kBackendClient;
    pub use sbs5k_backend_server::{Sbs5kBackend, Sbs5kBackendServer};
}

/// Types for dealing with blocks
pub mod block {
    tonic::include_proto!("sbs5k_messages.block");
}

// Types for dealing with chunks
pub mod chunk {
    tonic::include_proto!("sbs5k_messages.chunk");
}

/// Types for events that can happen in the game
pub mod events {
    tonic::include_proto!("sbs5k_messages.events");
}

/// Types for entities in the game world
pub mod world {
    tonic::include_proto!("sbs5k_messages.world");
}
