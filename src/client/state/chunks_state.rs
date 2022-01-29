use crate::client::constants;
use crate::client::mesh_generation::MeshGenerator;
use crate::engine::SceneObject;
use crate::world::chunk::{Chunk, ChunkIndex, ChunkSource};

/// A chunk that has been loaded into memory
pub(crate) struct LoadedChunk {
    pub index: ChunkIndex,
    pub chunk: Chunk,
}

/// A wrapper struct to encode all state relating to the management of chunks in the client
pub(crate) struct ChunksState {
    chunk_source: Box<dyn ChunkSource>,
    loaded_chunks: Vec<LoadedChunk>,
    renderable_chunks: Vec<SceneObject>,
    mesh_generator: MeshGenerator,
    current_chunk_index: ChunkIndex,
}

impl ChunksState {
    pub(crate) fn new(chunk_source: Box<dyn ChunkSource>, current_chunk_index: ChunkIndex) -> Self {
        let loaded_chunks = load_initial_chunks(current_chunk_index, chunk_source.as_ref());
        let mesh_generator = MeshGenerator::new();
        let renderable_chunks = loaded_chunks
            .iter()
            .map(|loaded_chunk| {
                mesh_generator.chunk_to_scene_object(&loaded_chunk.chunk, loaded_chunk.index)
            })
            .collect();

        ChunksState {
            chunk_source,
            loaded_chunks,
            renderable_chunks,
            mesh_generator,
            current_chunk_index,
        }
    }

    pub(crate) fn renderable_chunks(&self) -> &Vec<SceneObject> {
        &self.renderable_chunks
    }

    pub(crate) fn notify_player_changed_chunk(&mut self, new_chunk_index: ChunkIndex) {
        let indices_to_load =
            compute_chunk_indices_to_load(self.current_chunk_index, new_chunk_index);

        for chunk_index in indices_to_load {
            let position = get_chunk_position(chunk_index);
            let chunk = self.chunk_source.get_chunk_at(chunk_index);
            // TODO: Do this asynchronously
            let mesh = self
                .mesh_generator
                .chunk_to_scene_object(&chunk, chunk_index);

            let index_replaced = &self.loaded_chunks[position];
            println!("Replacing chunk at index ({}, {}) with chunk at index ({}, {})", index_replaced.index.i, index_replaced.index.j, chunk_index.i, chunk_index.j);

            self.loaded_chunks[position] = LoadedChunk {
                index: chunk_index,
                chunk,
            };
            self.renderable_chunks[position] = mesh;
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn chunk_at_index(&self, chunk_index: ChunkIndex) -> &Chunk {
        let index = get_chunk_position(chunk_index);
        &self.loaded_chunks[index].chunk
    }
}

/// Load the initial set of chunks around the player into the chunks buffer
fn load_initial_chunks(
    current_chunk_index: ChunkIndex,
    chunk_source: &dyn ChunkSource,
) -> Vec<LoadedChunk> {
    let mut chunks = Vec::with_capacity(constants::NUM_RENDERABLE_CHUNKS);
    let (min_chunk_index, max_chunk_index) = renderable_chunk_indices_range(current_chunk_index);
    for i in min_chunk_index.i..max_chunk_index.i + 1 as i32 {
        for j in min_chunk_index.j..max_chunk_index.j + 1 as i32 {
            let chunk_index = ChunkIndex { i, j };
            let loaded_chunk = LoadedChunk {
                index: chunk_index,
                chunk: chunk_source.get_chunk_at(chunk_index),
            };
            chunks.push(loaded_chunk);
        }
    }
    chunks
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
    for i in min_chunk.i..max_chunk.i+1 {
        for j in min_chunk.j..max_chunk.j+1 {
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
    let min_i = current_chunk_index.i - constants::RENDER_DISTANCE_CHUNKS as i32 + 1;
    let min_j = current_chunk_index.j - constants::RENDER_DISTANCE_CHUNKS as i32 + 1;
    let max_i = current_chunk_index.i + constants::RENDER_DISTANCE_CHUNKS as i32 - 1;
    let max_j = current_chunk_index.j + constants::RENDER_DISTANCE_CHUNKS as i32 - 1;
    let min = ChunkIndex { i: min_i, j: min_j };
    let max = ChunkIndex { i: max_i, j: max_j };
    (min, max)
}

/// Compute the value of `a` modulo `n`
///
/// This function is necessary because Rust's built-in modulo operator doesn't behave in a
/// mathematically correct fashion for negative values of `a`.
#[inline]
fn modulo(a: i32, n: i32) -> i32 {
    if a == 0 {
        0
    }
    else if a > 0 {
        a % n
    } else {
        n - ((-a) % n)
    }
}

/// Given a 2D chunk index, computes its position in a 1D array of chunks
#[inline]
fn get_chunk_position(chunk_index: ChunkIndex) -> usize {
    let square_edge_length = 2 * constants::RENDER_DISTANCE_CHUNKS - 1;
    (modulo(chunk_index.i, square_edge_length as i32) * square_edge_length as i32
        + modulo(chunk_index.j, square_edge_length as i32)) as usize
}

/// Determine whether a chunk index lies within a range
#[inline]
fn index_is_in_range(index: ChunkIndex, range: (ChunkIndex, ChunkIndex)) -> bool {
    let (lower, upper) = range;
    let i_in_range = index.i >= lower.i && index.i <= upper.i;
    let j_in_range = index.j >= lower.j && index.j <= upper.j;
    i_in_range && j_in_range
}
