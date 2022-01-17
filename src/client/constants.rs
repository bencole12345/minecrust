/// The "radius" of the square of renderable chunks
pub const RENDER_DISTANCE_CHUNKS: u32 = 3;

/// The length of the side of the square of renderable chunks, in chunk lengths
pub const RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE: u32 = 1 + 2 * RENDER_DISTANCE_CHUNKS;

/// The number of chunks that will be renderable at any point in time
///
/// This is area of the square of renderable chunks, measured as a number of chunks
pub const NUM_RENDERABLE_CHUNKS: u32 =
    RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE * RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE;
