use std::f32::consts::PI;
use std::sync::{Arc, RwLock};

use nalgebra::Point3;

use sbs5k_core::entity::EntityPosition;

use crate::state::chunks_state::ChunksState;
use crate::Args;

/// The client's view of the world's state
pub(crate) struct ClientState {
    /// The player's current position in the world
    pub player_position: EntityPosition,

    /// The state of the currently-loaded chunks
    pub chunks_state: ChunksState,

    /// Whether the game is currently "live". This is expected to be `true` until we enter the
    /// shutdown phase. May be accessed by multiple threads.
    pub is_live: Arc<RwLock<bool>>,
}

impl ClientState {
    pub(crate) fn new(config: &Args) -> Self {
        let player_position = EntityPosition {
            position: Point3::new(8.0, 66.0, 8.0),
            yaw: PI,
            pitch: 0.0,
            roll: 0.0,
        };

        let chunks_state = ChunksState::new(config.render_distance);
        let is_live = Arc::new(RwLock::new(true));

        ClientState {
            player_position,
            chunks_state,
            is_live,
        }
    }

    pub(crate) fn mark_dead(&self) {
        *self.is_live.write().unwrap() = false;
    }
}
