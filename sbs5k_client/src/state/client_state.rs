use std::f32::consts::PI;
use std::sync::{Arc, RwLock};

use nalgebra::Point3;

use sbs5k_world::entity::EntityPosition;

use super::chunks_state::ChunksState;

/// The client's view of the world's state
pub(crate) struct ClientState {
    pub player_position: EntityPosition,
    pub chunks_state: ChunksState,
    pub is_live: Arc<RwLock<bool>>,
}

impl ClientState {
    pub(crate) fn mark_dead(&self) {
        *self.is_live.write().unwrap() = false;
    }
}

impl Default for ClientState {
    fn default() -> Self {
        let player_position = EntityPosition {
            position: Point3::new(8.0, 66.0, 8.0),
            yaw: PI,
            pitch: 0.0,
            roll: 0.0,
        };

        let chunks_state = ChunksState::default();
        let is_live = Arc::new(RwLock::new(true));

        ClientState {
            player_position,
            chunks_state,
            is_live,
        }
    }
}
