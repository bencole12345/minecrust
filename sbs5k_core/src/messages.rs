use serde::{Deserialize, Serialize};

pub mod request {
    use crate::chunk;
    use crate::player;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct ConnectRequest {
        pub username: [u8; 32],
    }

    #[derive(Serialize, Deserialize)]
    pub struct DisconnectRequest {
        pub player_id: player::PlayerId,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LoadChunkRequest {
        pub chunk_index: chunk::ChunkCoordinate,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LoadChunkRangeRequest {
        pub chunk_index_begin: chunk::ChunkCoordinate,
        pub chunk_index_end: chunk::ChunkCoordinate,
    }
}

pub mod response {
    use crate::{chunk, player};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct ConnectResponse {
        pub player_id: player::PlayerId, // TODO: Summary of the current game
    }

    // Note: No response for disconnections

    #[derive(Serialize, Deserialize)]
    pub struct LoadChunkResponse {
        pub chunk_index: chunk::ChunkCoordinate,
        pub data: chunk::Chunk,
    }
}

pub mod event {
    use crate::entity;
    use crate::player;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct PlayerMoveEvent {
        pub player_id: player::PlayerId,
        pub new_position: entity::EntityPosition,
    }

    /// Announces that a player has joined the server
    pub struct PlayerJoinEvent {
        pub player_id: player::PlayerId,
        pub username: String,
    }

    /// Announces that a player has disconnected from the server
    #[derive(Serialize, Deserialize)]
    pub struct PlayerLeaveEvent {
        pub player_id: player::PlayerId,
    }

    // TODO: BlockChangedEvent

    #[derive(Serialize, Deserialize)]
    pub enum GameEvent {
        PlayerMove(PlayerMoveEvent),
    }
}

#[derive(Serialize, Deserialize)]
pub enum Client2ServerMsg {
    Connect(request::ConnectRequest),
    Disconnect(request::DisconnectRequest),
    LoadChunk(request::LoadChunkRequest),
    LoadChunkRange(request::LoadChunkRangeRequest),
    PlayerMove(event::PlayerMoveEvent),
}

enum MessageTypeId {
    Type1,
    Type2,
}

trait Client2ServerTrait {
    const MESSAGE_TYPE_ID: MessageTypeId;
}

impl Client2ServerTrait for request::ConnectRequest {
    const MESSAGE_TYPE_ID: MessageTypeId = MessageTypeId::Type1;
}

#[derive(Serialize, Deserialize)]
pub enum Server2ClientMsg {
    LoadChunkResult(response::LoadChunkResponse),
    PlayerMove(event::PlayerMoveEvent),
}

// #[derive(Serialize, Deserialize)]
// pub struct Client2Server {
//     pub sequence_num: u64,
//     pub message: Client2ServerMsg,
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct Server2Client {
//     pub sequence_num: u64,
//     pub message: Server2ClientMsg,
// }

// TODO: Find a way to assert statically that Client2Server and Server2Client are both no larger than MTU
