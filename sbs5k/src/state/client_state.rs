use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use nalgebra as na;

use sbs5k_core::geometry;

use crate::state::chunks_state::ChunksState;
use crate::Args;

/// The game client's view of the world's state
pub(crate) struct GameClientState {
    /// The player's current position in the world
    pub player_position: Rc<RefCell<geometry::EntityPosition>>,

    /// The state of the currently-loaded chunks
    pub chunks_state: Rc<RefCell<ChunksState>>,

    /// Whether the game is currently "live". This is expected to be `true` until we enter the
    /// shutdown phase. May be accessed by multiple threads.
    pub is_live: Arc<RwLock<bool>>,
}

impl GameClientState {
    pub(crate) fn new(config: &Args) -> Self {
        let location = na::Point3::new(8.0, 66.0, 8.0);
        let orientation = geometry::Orientation {
            yaw: PI,
            pitch: 0.0,
            roll: 0.0,
        };
        let player_position = Rc::new(RefCell::new(geometry::EntityPosition {
            location,
            orientation,
        }));

        let chunks_state = Rc::new(RefCell::new(ChunksState::new(config.render_distance)));
        let is_live = Arc::new(RwLock::new(true));

        GameClientState {
            player_position,
            chunks_state,
            is_live,
        }
    }

    pub(crate) fn mark_dead(&self) {
        *self.is_live.write().unwrap() = false;
    }
}
