// TODO: Eventually this will be a proper file-based store, but for now let's just keep it in-memory
//
// So: it will persist between player disconnections and reconnections, but not restarts of the server.

use std::collections::HashMap;

use crate::state::PlayerState;

use sbs5k_core::chunk;

pub(crate) trait BackingStore {
    fn load_player_state(&self, username: &str) -> Option<PlayerState>;
    fn save_player_state(&mut self, state: PlayerState);

    fn load_chunk(&self, coord: chunk::ChunkCoordinate) -> Option<&Box<chunk::Chunk>>;
    fn save_chunk(&mut self, coord: chunk::ChunkCoordinate, chunk: Box<chunk::Chunk>);
}

#[derive(Default)]
pub(crate) struct InMemoryBackingStore {
    player_states: HashMap<String, PlayerState>,
    chunks: HashMap<chunk::ChunkCoordinate, Box<chunk::Chunk>>,
}

impl BackingStore for InMemoryBackingStore {
    fn load_player_state(&self, username: &str) -> Option<PlayerState> {
        self.player_states.get(username).cloned()
    }

    fn save_player_state(&mut self, state: PlayerState) {
        self.player_states.insert(state.username.clone(), state);
    }

    fn load_chunk(&self, coord: chunk::ChunkCoordinate) -> Option<&Box<chunk::Chunk>> {
        self.chunks.get(&coord)
    }

    fn save_chunk(&mut self, coord: chunk::ChunkCoordinate, chunk: Box<chunk::Chunk>) {
        self.chunks.insert(coord, chunk);
    }
}
