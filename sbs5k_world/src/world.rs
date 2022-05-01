use crate::world::chunk::Chunk;

/// Encodes the state of everything in the game world
///
/// TODO: Make this serialisable using Serde
pub struct World {
    pub chunks: Vec<Chunk>,
}
