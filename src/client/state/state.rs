use std::f32::consts::PI;

use crate::client::state::chunks_state::ChunksState;
use crate::world::entity::EntityPosition;

/// The client's view of the world's state
pub(crate) struct ClientState {
    pub player_position: EntityPosition,
    pub chunks_state: ChunksState,
}

impl ClientState {
    pub(crate) fn new() -> Self {
        let player_position = EntityPosition {
            position: na::Point3::new(8.0, 66.0, 8.0),
            yaw: PI,
            pitch: 0.0,
            roll: 0.0,
        };

        let chunks_state = ChunksState::new();

        ClientState {
            player_position,
            chunks_state,
        }
    }
}
