use nalgebra::Point3;
use serde;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::block::Block;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

pub const BLOCKS_IN_CHUNK: usize = CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT;

/// The blocks that comprise one chunk.
///
/// Blocks should be stored in X-major, followed by Y-major, order.
pub type ChunkBlocks = [Block; BLOCKS_IN_CHUNK];

/// A 16x16x256 volume of space
// TODO: Make this representation more efficient. Right now it's 64K
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chunk {
    /// The blocks contained in this chunk
    #[serde(with = "BigArray")]
    blocks: ChunkBlocks,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            blocks: empty_blocks(),
        }
    }
}

/// The unique 2D integral coordinate of a chunk
///
/// The `i` coordinate corresponds to its position in the x dimension; the `j` coordinate
/// corresponds to its position in the z dimension. The index (0, 0) is the chunk that the player
/// first spawns in.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChunkCoordinate {
    pub i: i32,
    pub j: i32,
}

/// A source of chunks guaranteed to work for any possible chunk index
///
/// Possible implementations may include loading chunks from a file or over a network.
pub trait ChunkSource {
    fn get_chunk_at(&mut self, coordinate: ChunkCoordinate) -> Box<Chunk>;
}

// TODO: Define a type for PlayerPosition? (WorldPosition?)
// TODO: Use the From trait for this?
impl ChunkCoordinate {
    pub fn from_player_position(player_position: Point3<f32>) -> Self {
        let i = if player_position.x >= 0.0 {
            (player_position.x / (CHUNK_WIDTH as f32)) as i32
        } else {
            (player_position.x / (CHUNK_WIDTH as f32)) as i32 - 1
        };
        let j = if player_position.z >= 0.0 {
            (player_position.z / (CHUNK_DEPTH as f32)) as i32
        } else {
            (player_position.z / (CHUNK_DEPTH as f32)) as i32 - 1
        };
        ChunkCoordinate { i, j }
    }
}

pub fn empty_blocks() -> ChunkBlocks {
    [Block::Empty; BLOCKS_IN_CHUNK]
}

#[inline(always)]
fn block_index(x: usize, y: usize, z: usize) -> usize {
    (CHUNK_HEIGHT * CHUNK_DEPTH) * x + (CHUNK_WIDTH) * y + z
}

impl Chunk {
    pub fn new(blocks: ChunkBlocks) -> Self {
        Chunk { blocks }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: Block) {
        let idx = block_index(x, y, z);
        self.blocks[idx] = block;
    }

    #[inline(always)]
    pub fn get_block_at(&self, x: usize, y: usize, z: usize) -> Block {
        let idx = block_index(x, y, z);
        self.blocks[idx]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ChunkLoadResult {
    pub chunk: Box<Chunk>,
    pub coordinate: ChunkCoordinate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point3;
    use rstest::*;

    #[rstest]
    #[case(0, 0, 0, 0)]
    #[case(1, 0, 0, CHUNK_HEIGHT*CHUNK_DEPTH)]
    #[case(0, 1, 0, CHUNK_DEPTH)]
    #[case(0, 0, 1, 1)]
    #[case(1, 2, 3, 1*CHUNK_HEIGHT*CHUNK_DEPTH + 2*CHUNK_DEPTH + 3)]
    fn block_index_works(
        #[case] x: usize,
        #[case] y: usize,
        #[case] z: usize,
        #[case] expected_idx: usize,
    ) {
        let idx = block_index(x, y, z);
        assert_eq!(expected_idx, idx);
    }

    #[rstest]
    #[case(Point3::new(1.0, 0.0, 1.0), ChunkCoordinate{i: 0, j: 0})]
    #[case(Point3::new(1.0, 64.0, 1.0), ChunkCoordinate{i: 0, j: 0})]
    #[case(Point3::new(8.0, 0.0, 1.0), ChunkCoordinate{i: 0, j: 0})]
    #[case(Point3::new(16.1, 0.0, 0.0), ChunkCoordinate{i: 1, j: 0})]
    #[case(Point3::new(0.0, 0.0, - 16.1), ChunkCoordinate{i: 0, j: - 2})]
    #[case(Point3::new(0.0, 0.0, 16.1), ChunkCoordinate{i: 0, j: 1})]
    #[case(Point3::new(8.0, 66.0, - 8.0), ChunkCoordinate{i: 0, j: - 1})]
    #[case(Point3::new(17.0, 66.0, - 8.0), ChunkCoordinate{i: 1, j: - 1})]
    fn chunkcoordinate_from_player_position_works(
        #[case] player_pos: Point3<f32>,
        #[case] expected_index: ChunkCoordinate,
    ) {
        let result = ChunkCoordinate::from_player_position(player_pos);
        assert_eq!(expected_index, result);
    }
}
