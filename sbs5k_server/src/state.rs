use sbs5k_core::backend_api::PlayerID;
use sbs5k_core::geometry;
use std::collections::HashMap;

/// Application-level state relating to a player
#[derive(Clone, Default)]
pub(crate) struct PlayerState {
    pub username: String,
    pub current_position: geometry::EntityPosition,
}

#[derive(Default)]
pub(crate) struct GlobalServerState {
    pub player_username_to_id: HashMap<String, PlayerID>,

    next_player_id: PlayerID,
}

impl GlobalServerState {
    pub fn assign_player_id(&mut self) -> PlayerID {
        // TODO: Consider using a smarter algorithm for this (e.g. repeatedly generate a UUID until
        // no collision with any in use)
        let assigned_id = self.next_player_id;
        self.next_player_id = if self.next_player_id == PlayerID::MAX {
            0
        } else {
            self.next_player_id + 1
        };
        assigned_id
    }
}
