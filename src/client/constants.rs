/// The "radius" of the square of renderable chunks
///
/// If the player is currently in chunk index (i, j), then all chunks from the square spanned by
/// `(i - RENDER_DISTANCE_CHUNKS, j - RENDER_DISTANCE_CHUNKS)` to
/// `(i + RENDER_DISTANCE_CHUNKS, j + RENDER_DISTANCE_CHUNKS)` (inclusive) will be rendered.
pub const RENDER_DISTANCE_CHUNKS: usize = 4;

/// The number of chunks that will be renderable at any point in time
pub const NUM_RENDERABLE_CHUNKS: usize = 4 * RENDER_DISTANCE_CHUNKS * RENDER_DISTANCE_CHUNKS;
