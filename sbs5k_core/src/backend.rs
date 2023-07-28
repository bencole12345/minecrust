use crate::{chunk, entity, messages};
use std::sync::mpsc;

pub type EventStream = mpsc::Receiver<messages::event::GameEvent>;

/// Interface for a SBS5K backend
pub trait Backend {
    /// Request all blocks between `start` and `end` (inclusive)
    fn request_blocks(&self, start: chunk::ChunkCoordinate, end: chunk::ChunkCoordinate);

    /// Notify the backend that the player's position has changed
    fn move_player(&self, new_position: entity::EntityPosition);
}
