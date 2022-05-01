use nalgebra::Point3;

use crate::block::Block;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

/// A 16x16x256 volume of space
#[derive(Copy, Clone, Debug)]
pub struct Chunk {
    /// The blocks contained in this chunk
    blocks: ChunkBlocks,
    // TODO: Compute lighting levels (don't serialise)
}

/// The unique 2D integral coordinate of a chunk
///
/// The `i` coordinate corresponds to its position in the x dimension; the `j` coordinate
/// corresponds to its position in the z dimension. The index (0, 0) is the chunk that the player
/// first spawns in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChunkCoordinate {
    pub i: i32,
    pub j: i32,
}

/// A source of chunks guaranteed to work for any possible chunk index
///
/// Possible implementations may include loading chunks from a file or over a network.
pub trait ChunkSource {
    fn get_chunk_at(&mut self, coordinate: ChunkCoordinate) -> Chunk;
}

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

pub type ChunkBlocks = [[[Block; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

pub fn empty_blocks() -> ChunkBlocks {
    [[[Block::Empty; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH]
}

impl Chunk {
    pub fn new(blocks: ChunkBlocks) -> Self {
        Chunk { blocks }
    }

    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point3;
    use rstest::*;

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
