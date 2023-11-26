use serde::{Deserialize, Serialize};

use crate::{chunk, geometry};

#[derive(Serialize, Deserialize, Debug)]
pub enum Client2Server {
    Connected,
    Heartbeat,
    Disconnected,

    NotifyNewLocation,

    LoadChunksPls,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Server2Client {
    OnConnected {
        start_location: geometry::Location,
    },
    ChunkLoadedUrWelcome {
        chunk: chunk::Chunk,
        coordinate: chunk::ChunkCoordinate,
    },
}

/// The primary message format from a client to the server
#[derive(Serialize, Deserialize, Debug)]
pub struct Client2ServerMessage {
    pub player_id: u32,
    pub message: Client2Server,
}

/// The primary message format from the server to a client
#[derive(Serialize, Deserialize, Debug)]
pub struct Server2ClientMessage {
    pub message: Server2Client,
}
