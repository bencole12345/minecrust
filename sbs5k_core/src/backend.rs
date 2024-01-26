use serde::{Deserialize, Serialize};

use crate::{chunk, geometry};

/// A token type offered by the server upon successful login. The client must include this token in all future requests: it uniquely identifies the session.
pub type Token = u64;

#[derive(Serialize, Deserialize, Debug)]
pub enum Client2Server {
    /// The initial login message
    Login {
        username: String,
        // TODO: Include some kind of identifier (git hash?) so we can detect mismatches
    },

    /// End the session. The client doesn't really care about this, but it lets the server free up resources without waiting for the client to time out
    Logout {
        token: Token,
    },

    Heartbeat {
        token: Token,
    },

    /// Inform the server that the player's location (position or orientation) has changed
    NotifyNewPosition {
        token: Token,
        position: geometry::EntityPosition,
    },

    /// Request to load a set of chunks
    LoadChunksPls {
        token: Token,
        coordinate: chunk::ChunkCoordinate,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Server2Client {
    /// Returned on receipt of a successful login
    OnLoginSuccess {
        token: Token,
        start_position: geometry::EntityPosition,
    },

    OnLoginRejected {
        reason: String,
    },

    OtherPlayerMoved {
        player_id: u32,
        new_position: geometry::EntityPosition,
    },

    ChunkLoadedUrWelcome {
        chunk: Box<chunk::Chunk>,
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
