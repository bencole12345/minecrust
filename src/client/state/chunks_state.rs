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
        }
    }

    pub(crate) fn renderable_chunks(&self) -> &Vec<SceneObject> {
        &self.renderable_chunks
    }

    pub(crate) fn notify_player_changed_chunk(&mut self, _new_chunk_index: ChunkIndex) {
        // TODO: Synchronously swap the loadable chunks
        // TODO: Asynchronously fetch more chunks
        todo!()
    }

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

/// Compute the range of chunks that should be renderable for a given player index
///
/// This is a square of dimensions (2*RENDER_DISTANCE_CHUNKS) * (2*RENDER_DISTANCE_CHUNKS), centred
/// around the chunk containing the player. In the event that RENDER_DISTANCE_CHUNKS is even, there
/// are essentially four middle chunks, in which case the player's chunk will be the one with the
/// largest `i` and `j` values.
fn renderable_chunk_indices_range(current_chunk_index: ChunkIndex) -> (ChunkIndex, ChunkIndex) {
    let min_i = current_chunk_index.i - constants::RENDER_DISTANCE_CHUNKS as i32;
    let min_j = current_chunk_index.j - constants::RENDER_DISTANCE_CHUNKS as i32;
    let max_i = current_chunk_index.i + constants::RENDER_DISTANCE_CHUNKS as i32 - 1;
    let max_j = current_chunk_index.j + constants::RENDER_DISTANCE_CHUNKS as i32 - 1;
    let min = ChunkIndex { i: min_i, j: min_j };
    let max = ChunkIndex { i: max_i, j: max_j };
    (min, max)
}

/// Given a 2D chunk index, computes its position in a 1D array of chunks
#[inline]
fn get_chunk_position(chunk_index: ChunkIndex) -> usize {
    let square_edge_length = constants::RENDER_DISTANCE_CHUNKS * 2;
    ((chunk_index.i % square_edge_length as i32) * square_edge_length as i32
        + (chunk_index.j % square_edge_length as i32)) as usize
}
