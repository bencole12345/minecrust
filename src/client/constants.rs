/// The "radius" of the square of renderable chunks
pub const RENDER_DISTANCE_CHUNKS: u32 = 3;

/// The length of the side of the square of renderable chunks, in chunk lengths
pub const RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE: u32 = 1 + 2 * RENDER_DISTANCE_CHUNKS;

/// The number of chunks that will be renderable at any point in time
///
/// This is area of the square of renderable chunks, measured as a number of chunks
pub const NUM_RENDERABLE_CHUNKS: u32 =
    RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE * RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE;

/// The movement speed of the player in blocks per second
pub const MOVE_SPEED: f32 = 4.0;

/// The turn speed of the player in radians per half-screen-width of mouse movement
pub const TURN_SENSITIVITY: f32 = 2.0;
