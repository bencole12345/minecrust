// TODO: Rename this file to backend_api
use serde::{Deserialize, Serialize};

use crate::{chunk, geometry};

/// A token type offered by the server upon successful login. The client must include this token in all future requests: it uniquely identifies the session.
pub type PlayerID = u64;

pub mod c2s {
    use super::*;

    /// The initial login message
    #[derive(Serialize, Deserialize, Debug)]
    pub struct LoginMsg {
        pub username: String,
        // TODO: Include some kind of identifier (git hash?) so we can detect mismatches
    }

    /// End the session. The client doesn't really care about this, but it lets the server free up resources without waiting for the client to time out
    #[derive(Serialize, Deserialize, Debug)]
    pub struct LogoutMsg {
        pub player_id: PlayerID,
    }

    /// Inform the server that the player's location (position or orientation) has changed
    #[derive(Serialize, Deserialize, Debug)]
    pub struct NotifyNewPositionMsg {
        pub player_id: PlayerID,
        pub position: geometry::EntityPosition,
    }

    /// Request to load a set of chunks
    #[derive(Serialize, Deserialize, Debug)]
    pub struct LoadChunksPlsMsg {
        pub token: PlayerID,
        pub coordinate: chunk::ChunkCoordinate,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Client2ServerMsg {
    Login(c2s::LoginMsg),
    Logout(c2s::LogoutMsg),
    NotifyNewPosition(c2s::NotifyNewPositionMsg),
    LoadChunksPls(c2s::LoadChunksPlsMsg),
}

impl Client2ServerMsg {
    pub fn to_buf(&self) -> Vec<u8> {
        // TODO: Consider allowing caller to pass in the buffer to use
        bincode::serialize(self).expect("Bad Client2Server message")
    }

    pub fn from_buf(buf: &[u8]) -> Self {
        bincode::deserialize(buf).expect("Bad Client2Server message")
    }
}

pub mod s2c {
    use super::*;

    /// Returned on receipt of a successful login
    #[derive(Serialize, Deserialize, Debug)]
    pub struct OnLoginSuccessMsg {
        pub token: PlayerID,
        pub start_position: geometry::EntityPosition,
    }

    /// Returned when a login is rejected, e.g. because that user is already logged in
    #[derive(Serialize, Deserialize, Debug)]
    pub struct OnLoginRejectedMsg {
        pub reason: String,
    }

    /// A chunk that was requested by a client
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChunkLoadedUrWelcomeMsg {
        pub chunk_load_result: chunk::ChunkLoadResult,
    }

    // TODO: Implement
    // #[derive(Serialize, Deserialize, Debug)]
    // pub struct OtherPlayerMovedMsg {
    //     player_id: u32,
    //     new_position: geometry::EntityPosition,
    // }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Server2ClientMsg {
    OnLoginSuccess(s2c::OnLoginSuccessMsg),
    OnLoginRejected(s2c::OnLoginRejectedMsg),
    ChunkLoadedUrWelcome(s2c::ChunkLoadedUrWelcomeMsg),
}

impl Server2ClientMsg {
    pub fn to_buf(&self) -> Vec<u8> {
        // TODO: Consider allowing caller to pass in the buffer to use
        bincode::serialize(self).expect("Bad Server2Client message")
    }

    pub fn from_buf(buf: &[u8]) -> Self {
        bincode::deserialize(buf).expect("Bad Server2Client message")
    }
}
