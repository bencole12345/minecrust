use sbs5k_engine::SceneObject;
use sbs5k_maths::modulo;
use sbs5k_world::chunk::{Chunk, ChunkCoordinate};

use crate::constants;

/// A wrapper struct to encode all state relating to the management of chunks in the client
pub(crate) struct ChunksState {
    chunks: Vec<Option<Chunk>>,
    chunk_meshes: Vec<Option<SceneObject>>,
}

impl ChunksState {
    pub(crate) fn renderable_chunks(&self) -> Vec<&SceneObject> {
        self.chunk_meshes
            .iter()
            .filter_map(|chunk| chunk.as_ref())
            .collect()
    }

    #[inline(always)]
    pub(crate) fn set_chunk(&mut self, chunk_coord: ChunkCoordinate, value: Option<Chunk>) {
        let index = get_chunk_index(chunk_coord);
        self.chunks[index] = value;
    }

    #[inline(always)]
    pub(crate) fn set_chunk_mesh(
        &mut self,
        chunk_coord: ChunkCoordinate,
        value: Option<SceneObject>,
    ) {
        let index = get_chunk_index(chunk_coord);
        self.chunk_meshes[index] = value;
    }
}

impl Default for ChunksState {
    fn default() -> Self {
        let mut chunks = vec![];
        chunks.resize_with(constants::NUM_RENDERABLE_CHUNKS as usize, || None);

        let mut chunk_meshes = vec![];
        chunk_meshes.resize_with(constants::NUM_RENDERABLE_CHUNKS as usize, || None);

        ChunksState {
            chunks,
            chunk_meshes,
        }
    }
}

#[inline(always)]
fn get_chunk_index(chunk_coord: ChunkCoordinate) -> usize {
    let i = modulo(chunk_coord.i, constants::RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE) as usize;
    let j = modulo(chunk_coord.j, constants::RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE) as usize;
    i * constants::RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE as usize + j
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(ChunkCoordinate{i: 0, j: 0}, 0)]
    fn default_chunk_position_works(#[case] chunk_coord: ChunkCoordinate, #[case] expected: usize) {
        let actual = get_chunk_index(chunk_coord);
        assert_eq!(expected, actual);
    }
}
