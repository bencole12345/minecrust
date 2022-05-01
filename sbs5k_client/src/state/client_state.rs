use std::f32::consts::PI;

use nalgebra::Point3;

use sbs5k_engine::SceneObject;
use sbs5k_world::chunk::{ChunkCoordinate, ChunkSource};
use sbs5k_world::entity::EntityPosition;

use super::chunks_state::ChunksState;

/// The client's view of the world's state
pub(crate) struct ClientState {
    pub player_position: EntityPosition,
    pub chunks_state: ChunksState,
}

impl ClientState {
    pub(crate) fn new(chunk_source: Box<dyn ChunkSource>) -> Self {
        let player_position = EntityPosition {
            position: Point3::new(8.0, 66.0, 8.0),
            yaw: PI,
            pitch: 0.0,
            roll: 0.0,
        };

        let current_chunk_index = ChunkCoordinate { i: 0, j: 0 };
        let chunks_state = ChunksState::new(chunk_source, current_chunk_index);

        ClientState {
            player_position,
            chunks_state,
        }
    }

    /// Update the state in response to a change in the player's current chunk index
    pub(crate) fn notify_player_changed_chunk(&mut self, new_chunk_index: ChunkCoordinate) {
        self.chunks_state
            .notify_player_changed_chunk(new_chunk_index);
    }

    /// The chunks currently inside the render distance
    pub(crate) fn renderable_chunks(&self) -> Vec<&SceneObject> {
        self.chunks_state.renderable_chunks()
    }
}
