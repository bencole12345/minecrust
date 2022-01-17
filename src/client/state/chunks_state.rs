use crate::client::constants::{
    NUM_RENDERABLE_CHUNKS, RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE, RENDER_DISTANCE_CHUNKS,
};
use crate::client::mesh_generation::MeshGenerator;
use crate::client::util::modulo;
use crate::engine::SceneObject;
use crate::world::chunk::{Chunk, ChunkIndex, ChunkSource};

type LoadedChunksState = [[Option<Chunk>; RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE as usize];
    RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE as usize];
type RenderableChunksState = [[Option<SceneObject>; RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE as usize];
    RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE as usize];

/// A wrapper struct to encode all state relating to the management of chunks in the client
pub(crate) struct ChunksState {
    chunks: Box<LoadedChunksState>,
    chunk_meshes: Box<RenderableChunksState>,
    mesh_generator: MeshGenerator,
    chunk_source: Box<dyn ChunkSource>,
    current_chunk_index: ChunkIndex,
}

impl ChunksState {
    pub(crate) fn new(chunk_source: Box<dyn ChunkSource>, current_chunk_index: ChunkIndex) -> Self {
        ChunksState {
            chunks: Default::default(),
            chunk_meshes: Default::default(),
            mesh_generator: MeshGenerator::new(),
            chunk_source,
            current_chunk_index,
        }
    }

    pub(crate) fn renderable_chunks(&self) -> Vec<&SceneObject> {
        // TODO: Optimise, maybe make into an iterator
        let mut renderable = vec![];
        renderable.reserve(NUM_RENDERABLE_CHUNKS as usize);
        for row in &*self.chunk_meshes {
            for chunk_scene_object in row {
                if let Some(object) = chunk_scene_object {
                    renderable.push(object);
                }
            }
        }
        renderable
    }

    pub(crate) fn initialise_loaded_chunks(&mut self) {
        let (min_chunk_index, max_chunk_index) =
            renderable_chunk_indices_range(self.current_chunk_index);

        // Load each chunk
        // TODO: Generate meshes asynchronously
        for i in min_chunk_index.i..=max_chunk_index.i as i32 {
            for j in min_chunk_index.j..=max_chunk_index.j as i32 {
                let chunk_index = ChunkIndex { i, j };
                let (i_pos, j_pos) = get_chunk_position(&chunk_index);
                let chunk = self.chunk_source.get_chunk_at(&chunk_index);
                let mesh = self
                    .mesh_generator
                    .chunk_to_scene_object(&chunk, &chunk_index);
                self.chunks[i_pos][j_pos] = Some(chunk);
                self.chunk_meshes[i_pos][j_pos] = Some(mesh);
            }
        }
    }

    pub(crate) fn notify_player_changed_chunk(&mut self, new_chunk_index: ChunkIndex) {
        let indices_to_load =
            compute_chunk_indices_to_load(self.current_chunk_index, new_chunk_index);

        // Invalidate the old chunks
        for chunk_index in &indices_to_load {
            let (i, j) = get_chunk_position(chunk_index);
            self.chunks[i][j] = None;
            self.chunk_meshes[i][j] = None;
        }

        // Load the new chunks
        // TODO: Make this asynchronous
        for chunk_index in &indices_to_load {
            let (i, j) = get_chunk_position(chunk_index);
            let chunk = self.chunk_source.get_chunk_at(chunk_index);
            let mesh = self
                .mesh_generator
                .chunk_to_scene_object(&chunk, chunk_index);
            self.chunks[i][j] = Some(chunk);
            self.chunk_meshes[i][j] = Some(mesh);
        }

        self.current_chunk_index = new_chunk_index;
    }
}

/// Compute the set of chunk indices that must be loaded (and the ones in their places dropped) if
/// the player moved from `old_chunk_index` to `new_chunk_index`
fn compute_chunk_indices_to_load(
    old_chunk_index: ChunkIndex,
    new_chunk_index: ChunkIndex,
) -> Vec<ChunkIndex> {
    let range_before = renderable_chunk_indices_range(old_chunk_index);
    let range_after = renderable_chunk_indices_range(new_chunk_index);
    let (min_chunk, max_chunk) = range_after;

    let mut indices = vec![];
    for i in min_chunk.i..=max_chunk.i {
        for j in min_chunk.j..=max_chunk.j {
            let index = ChunkIndex { i, j };
            if !index_is_in_range(index, range_before) {
                indices.push(index);
            }
        }
    }
    indices
}

/// Compute the range of chunks that should be renderable for a given player index
///
/// This is a square of dimensions (1 + 2*RENDER_DISTANCE_CHUNKS) * (1 + 2*RENDER_DISTANCE_CHUNKS)
/// centred around the player.
fn renderable_chunk_indices_range(current_chunk_index: ChunkIndex) -> (ChunkIndex, ChunkIndex) {
    let min_i = current_chunk_index.i - RENDER_DISTANCE_CHUNKS as i32;
    let min_j = current_chunk_index.j - RENDER_DISTANCE_CHUNKS as i32;
    let max_i = current_chunk_index.i + RENDER_DISTANCE_CHUNKS as i32;
    let max_j = current_chunk_index.j + RENDER_DISTANCE_CHUNKS as i32;
    let min = ChunkIndex { i: min_i, j: min_j };
    let max = ChunkIndex { i: max_i, j: max_j };
    (min, max)
}

#[inline]
fn get_chunk_position(chunk_index: &ChunkIndex) -> (usize, usize) {
    let i = modulo(chunk_index.i, RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE);
    let j = modulo(chunk_index.j, RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE);
    (i as usize, j as usize)
}

/// Determine whether a chunk index lies within a range
#[inline]
fn index_is_in_range(index: ChunkIndex, range: (ChunkIndex, ChunkIndex)) -> bool {
    let (lower, upper) = range;
    let i_in_range = index.i >= lower.i && index.i <= upper.i;
    let j_in_range = index.j >= lower.j && index.j <= upper.j;
    i_in_range && j_in_range
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(ChunkIndex{i: 0, j: 0},
           ChunkIndex{i: -(RENDER_DISTANCE_CHUNKS as i32), j: -(RENDER_DISTANCE_CHUNKS as i32)},
           ChunkIndex{i: (RENDER_DISTANCE_CHUNKS as i32), j:(RENDER_DISTANCE_CHUNKS as i32)})]
    fn renderable_chunk_range_works(
        #[case] index: ChunkIndex,
        #[case] expected_min: ChunkIndex,
        #[case] expected_max: ChunkIndex,
    ) {
        let (actual_min, actual_max) = renderable_chunk_indices_range(index);
        assert_eq!(expected_min, actual_min);
        assert_eq!(expected_max, actual_max);
    }

    #[rstest]
    #[case(ChunkIndex{i: 0, j: 0}, (0, 0))]
    fn default_chunk_position_works(#[case] index: ChunkIndex, #[case] expected: (usize, usize)) {
        let actual = get_chunk_position(&index);
        assert_eq!(expected, actual);
    }
}
