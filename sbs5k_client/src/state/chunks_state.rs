use sbs5k_core::chunk::{Chunk, ChunkCoordinate};
use sbs5k_core::maths::modulo;
use sbs5k_engine::SceneObject;

/// A wrapper struct to encode all state relating to the management of chunks in the client
pub(crate) struct ChunksState {
    renderable_chunks_square_edge_size: u32,
    chunks: Vec<Option<Chunk>>,
    chunk_meshes: Vec<Option<SceneObject>>,
}

impl ChunksState {
    pub(crate) fn new(render_distance: u32) -> Self {
        let renderable_chunks_square_edge_size = 1 + 2 * render_distance;
        let num_renderable_chunks = renderable_chunks_square_edge_size * renderable_chunks_square_edge_size;

        let mut chunks = vec![];
        chunks.resize_with(num_renderable_chunks as usize, || None);

        let mut chunk_meshes = vec![];
        chunk_meshes.resize_with(num_renderable_chunks as usize, || None);

        ChunksState {
            renderable_chunks_square_edge_size,
            chunks,
            chunk_meshes,
        }
    }

    pub(crate) fn renderable_chunks(&self) -> Vec<&SceneObject> {
        self.chunk_meshes
            .iter()
            .filter_map(|chunk| chunk.as_ref())
            .collect()
    }

    #[inline(always)]
    pub(crate) fn set_chunk(&mut self, chunk_coord: ChunkCoordinate, value: Option<Chunk>) {
        let index = get_chunk_index(chunk_coord, self.renderable_chunks_square_edge_size);
        self.chunks[index] = value;
    }

    #[inline(always)]
    pub(crate) fn set_chunk_mesh(
        &mut self,
        chunk_coord: ChunkCoordinate,
        value: Option<SceneObject>,
    ) {
        let index = get_chunk_index(chunk_coord, self.renderable_chunks_square_edge_size);
        self.chunk_meshes[index] = value;
    }
}

#[inline(always)]
fn get_chunk_index(chunk_coord: ChunkCoordinate, edge_length: u32) -> usize {
    let i = modulo(chunk_coord.i, edge_length) as usize;
    let j = modulo(chunk_coord.j, edge_length) as usize;
    i * edge_length as usize + j
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    // TODO: Fix this test
    // TODO: Write more tests now that edge_length is injected

    #[rstest]
    #[case(ChunkCoordinate{i: 0, j: 0}, 1, 0)]
    fn default_chunk_position_works(#[case] chunk_coord: ChunkCoordinate, #[case] edge_length: u32, #[case] expected: usize) {
        let actual = get_chunk_index(chunk_coord, edge_length);
        assert_eq!(expected, actual);
    }
}
