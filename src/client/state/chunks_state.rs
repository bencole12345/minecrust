use crate::client::constants;
use crate::client::mesh_generation::MeshGenerator;
use crate::engine::SceneObject;
use crate::utils::maths::modulo;
use crate::world::chunk::{Chunk, ChunkCoordinate, ChunkSource};

/// A wrapper struct to encode all state relating to the management of chunks in the client
pub(crate) struct ChunksState {
    chunks: Vec<Option<Chunk>>,
    chunk_meshes: Vec<Option<SceneObject>>,
    mesh_generator: MeshGenerator,
    chunk_source: Box<dyn ChunkSource>,
    current_chunk_coord: ChunkCoordinate,
}

impl ChunksState {
    pub(crate) fn new(
        chunk_source: Box<dyn ChunkSource>,
        current_chunk_coord: ChunkCoordinate,
    ) -> Self {
        let mut chunks = vec![];
        chunks.resize_with(constants::NUM_RENDERABLE_CHUNKS as usize, || None);

        let mut chunk_meshes = vec![];
        chunk_meshes.resize_with(constants::NUM_RENDERABLE_CHUNKS as usize, || None);

        let mesh_generator = MeshGenerator::new();

        ChunksState {
            chunks,
            chunk_meshes,
            mesh_generator,
            chunk_source,
            current_chunk_coord,
        }
    }

    pub(crate) fn renderable_chunks(&self) -> Vec<&SceneObject> {
        self.chunk_meshes
            .iter()
            .filter_map(|chunk| chunk.as_ref())
            .collect()
    }

    pub(crate) fn initialise_loaded_chunks(&mut self) {
        // TODO: See if this can be integrated better with notify_player_changed_chunk

        let (min_chunk_coord, max_chunk_coord) =
            renderable_chunk_indices_range(self.current_chunk_coord);

        // Load each chunk
        // TODO: Generate meshes asynchronously
        for i in min_chunk_coord.i..=max_chunk_coord.i as i32 {
            for j in min_chunk_coord.j..=max_chunk_coord.j as i32 {
                let chunk_coord = ChunkCoordinate { i, j };
                let chunk = self.chunk_source.get_chunk_at(chunk_coord);
                let mesh = self
                    .mesh_generator
                    .chunk_to_scene_object(&chunk, chunk_coord);
                self.set_chunk(chunk_coord, Some(chunk));
                self.set_chunk_mesh(chunk_coord, Some(mesh));
            }
        }
    }

    pub(crate) fn notify_player_changed_chunk(&mut self, new_chunk_coord: ChunkCoordinate) {
        let chunk_coordinates_to_load = compute_chunks_to_load_after_player_current_chunk_change(
            self.current_chunk_coord,
            new_chunk_coord,
        );

        // Invalidate the old chunks
        for chunk_coord in &chunk_coordinates_to_load {
            self.set_chunk(*chunk_coord, None);
            self.set_chunk_mesh(*chunk_coord, None);
        }

        // Load the new chunks
        // TODO: Make this asynchronous
        for chunk_coord in chunk_coordinates_to_load {
            let chunk = self.chunk_source.get_chunk_at(chunk_coord);
            let mesh = self
                .mesh_generator
                .chunk_to_scene_object(&chunk, chunk_coord);
            self.set_chunk(chunk_coord, Some(chunk));
            self.set_chunk_mesh(chunk_coord, Some(mesh));
        }

        self.current_chunk_coord = new_chunk_coord;
    }

    #[inline(always)]
    fn set_chunk(&mut self, chunk_coord: ChunkCoordinate, value: Option<Chunk>) {
        let index = get_chunk_index(chunk_coord);
        self.chunks[index] = value;
    }

    #[inline(always)]
    fn set_chunk_mesh(&mut self, chunk_coord: ChunkCoordinate, value: Option<SceneObject>) {
        let index = get_chunk_index(chunk_coord);
        self.chunk_meshes[index] = value;
    }
}

/// Compute the set of chunk coordinates that must be loaded (and the ones in their places dropped)
/// if the player moved from `old_chunk_coord` to `new_chunk_coord`
fn compute_chunks_to_load_after_player_current_chunk_change(
    old_chunk_coord: ChunkCoordinate,
    new_chunk_coord: ChunkCoordinate,
) -> Vec<ChunkCoordinate> {
    let range_before = renderable_chunk_indices_range(old_chunk_coord);
    let range_after = renderable_chunk_indices_range(new_chunk_coord);
    let (min_chunk, max_chunk) = range_after;

    let mut coords = vec![];
    for i in min_chunk.i..=max_chunk.i {
        for j in min_chunk.j..=max_chunk.j {
            let coord = ChunkCoordinate { i, j };
            if !chunk_coordinate_is_in_range(coord, range_before) {
                coords.push(coord);
            }
        }
    }
    coords
}

/// Compute the range of chunks that should be renderable for a given player index
fn renderable_chunk_indices_range(
    current_chunk_coord: ChunkCoordinate,
) -> (ChunkCoordinate, ChunkCoordinate) {
    let min_i = current_chunk_coord.i - constants::RENDER_DISTANCE_CHUNKS as i32;
    let min_j = current_chunk_coord.j - constants::RENDER_DISTANCE_CHUNKS as i32;
    let max_i = current_chunk_coord.i + constants::RENDER_DISTANCE_CHUNKS as i32;
    let max_j = current_chunk_coord.j + constants::RENDER_DISTANCE_CHUNKS as i32;
    let min = ChunkCoordinate { i: min_i, j: min_j };
    let max = ChunkCoordinate { i: max_i, j: max_j };
    (min, max)
}

#[inline(always)]
fn get_chunk_index(chunk_coord: ChunkCoordinate) -> usize {
    let i = modulo(chunk_coord.i, constants::RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE) as usize;
    let j = modulo(chunk_coord.j, constants::RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE) as usize;
    i * constants::RENDERABLE_CHUNKS_SQUARE_EDGE_SIZE as usize + j
}

/// Determine whether a chunk coordinate lies within a given range
#[inline]
fn chunk_coordinate_is_in_range(
    index: ChunkCoordinate,
    range: (ChunkCoordinate, ChunkCoordinate),
) -> bool {
    let (lower, upper) = range;
    let i_in_range = index.i >= lower.i && index.i <= upper.i;
    let j_in_range = index.j >= lower.j && index.j <= upper.j;
    i_in_range && j_in_range
}

#[cfg(test)]
mod tests {
    use super::*;
    use constants::RENDER_DISTANCE_CHUNKS;
    use rstest::*;

    #[rstest]
    #[case(ChunkCoordinate{i: 0, j: 0},
    ChunkCoordinate{i: - (RENDER_DISTANCE_CHUNKS as i32), j: - (RENDER_DISTANCE_CHUNKS as i32)},
    ChunkCoordinate{i: (RENDER_DISTANCE_CHUNKS as i32), j: (RENDER_DISTANCE_CHUNKS as i32)})]
    fn renderable_chunk_range_works(
        #[case] chunk_coord: ChunkCoordinate,
        #[case] expected_min: ChunkCoordinate,
        #[case] expected_max: ChunkCoordinate,
    ) {
        let (actual_min, actual_max) = renderable_chunk_indices_range(chunk_coord);
        assert_eq!(expected_min, actual_min);
        assert_eq!(expected_max, actual_max);
    }

    #[rstest]
    #[case(ChunkCoordinate{i: 0, j: 0}, 0)]
    fn default_chunk_position_works(#[case] chunk_coord: ChunkCoordinate, #[case] expected: usize) {
        let actual = get_chunk_index(chunk_coord);
        assert_eq!(expected, actual);
    }
}
