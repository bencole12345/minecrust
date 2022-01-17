use crate::world::block::Block;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

/// A 16x16x256 volume of space
#[derive(Debug)]
pub struct Chunk {
    /// The blocks contained in this chunk
    blocks: ChunkBlocks,
    // TODO: Compute lighting levels (don't serialise)
}

#[derive(Clone, Copy, PartialEq)]
pub struct ChunkIndex {
    pub i: i32,
    pub j: i32,
}

/// A source of chunks guaranteed to work for any possible index.
///
/// Possible implementations may include loading chunks from a file or over a network.
pub trait ChunkSource {
    fn get_chunk_at(&self, index: ChunkIndex) -> Chunk;
}

impl ChunkIndex {
    pub fn from_player_position(player_position: na::Point3<f32>) -> Self {
        let i = (player_position.x / (CHUNK_WIDTH as f32)) as i32;
        let j = (player_position.z / (CHUNK_DEPTH as f32)) as i32;
        ChunkIndex { i, j }
    }
}

pub type ChunkBlocks = [[[Block; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

pub fn empty_blocks() -> ChunkBlocks {
    [[[Block::Empty; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH]
}

impl Chunk {
    pub fn new(blocks: ChunkBlocks) -> Self {
        Chunk { blocks }
    }

    #[inline]
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.blocks[x][y][z] = block;
    }

    #[inline]
    pub fn get_block_at(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[x][y][z]
    }

    #[inline]
    pub fn has_block_at(&self, x: i32, y: i32, z: i32) -> bool {
        let x_in_bounds = x >= 0 && x < CHUNK_WIDTH as i32;
        let y_in_bounds = y >= 0 && y < CHUNK_HEIGHT as i32;
        let z_in_bounds = z >= 0 && z < CHUNK_DEPTH as i32;
        if x_in_bounds && y_in_bounds && z_in_bounds {
            self.get_block_at(x as usize, y as usize, z as usize) != Block::Empty
        } else {
            false
        }
    }
}
