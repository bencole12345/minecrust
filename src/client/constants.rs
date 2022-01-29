/// The "radius" of the square of renderable chunks
///
/// The set of chunks that is rendered is a square of dimensions
/// (1 + 2*RENDER_DISTANCE_CHUNKS) * (1 + 2*RENDER_DISTANCE_CHUNKS), centred around the player's
/// current chunk.
pub const RENDER_DISTANCE_CHUNKS: usize = 2;

/// The number of chunks that will be renderable at any point in time
pub const NUM_RENDERABLE_CHUNKS: usize =
    (2 * RENDER_DISTANCE_CHUNKS - 1) * (2 * RENDER_DISTANCE_CHUNKS - 1);
