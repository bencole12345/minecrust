// Long-term, the plan is that a Chunk will cleverly compute just the
// external surface of polygons that need to be rendered and ignore everything
// else. At least initially, this structure can be computed when the chunk is
// generated and then completely recomputed whenever the chunk is modified.

use super::block::Block;

const BLOCK_WIDTH: usize = 16;
const BLOCK_DEPTH: usize = 16;
const BLOCK_HEIGHT: usize = 256;

/// A 16x16x256 volume of space
pub struct Chunk {
    /// The blocks contained in this chunk
    blocks: [[[Block; BLOCK_HEIGHT]; BLOCK_DEPTH]; BLOCK_WIDTH], // TODO: Add precomputed vertex data for only the exposed edges
}

impl Chunk {
    pub fn block_at(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[x][y][z]
    }
}
