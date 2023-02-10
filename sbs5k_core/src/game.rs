use crate::chunk::{Chunk, ChunkIndex};
use crate::event::Event;
use crate::stream::BiDirectionalStream;

/// A core trait capturing the requirements needed of an implementation of the game world.
///
/// Possible implementations might be a locally-running instance, or a network-based implementation that communicates
/// with a remote server via blocking calls.
pub trait World {
    /// TODO: Entrypoint to query the current (initial) state to find out the current players

    /// Obtain the chunks at a list of indices.
    fn get_chunks(indices: &[ChunkIndex]) -> Vec<Chunk>;

    /// Commence a bidirectional stream of events.
    fn establish_event_stream() -> BiDirectionalStream<Event, Event>;
}
