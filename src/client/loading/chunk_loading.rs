use std::sync::mpsc;

use crate::client::constants;
use crate::world::chunk::{Chunk, ChunkCoordinate, ChunkSource};

pub(crate) struct ChunkLoader {
    chunk_source: Box<dyn ChunkSource + Send>,
    current_chunk: ChunkCoordinate,
}

pub(crate) enum ChunkLoadRequest {
    InitialLoad(ChunkCoordinate),
    ChunkChangeLoad(ChunkCoordinate),
}

pub(crate) struct ChunkLoadResult {
    pub chunk: Box<Chunk>,
    pub coordinate: ChunkCoordinate,
}

type ChunkLoadRequestChannel = mpsc::Receiver<ChunkLoadRequest>;
type ChunkLoadResponseChannel = mpsc::Sender<ChunkLoadResult>;

impl ChunkLoader {
    pub(crate) fn new(chunk_source: Box<dyn ChunkSource + Send>) -> Self {
        Self {
            chunk_source,
            current_chunk: ChunkCoordinate { i: 0, j: 0 },
        }
    }

    pub(crate) fn generate_chunks(
        &mut self,
        requests_channel: ChunkLoadRequestChannel,
        mut responses_channel: ChunkLoadResponseChannel,
    ) {
        for chunk_load_request in requests_channel {
            match chunk_load_request {
                ChunkLoadRequest::InitialLoad(coordinate) => {
                    self.process_initial_player_coordinate(coordinate, &mut responses_channel);
                }

                ChunkLoadRequest::ChunkChangeLoad(new_chunk_coordinate) => {
                    self.process_new_player_coordinate(
                        new_chunk_coordinate,
                        &mut responses_channel,
                    );
                }
            }
        }
    }

    #[inline(always)]
    fn process_new_player_coordinate(
        &mut self,
        new_player_chunk_coordinate: ChunkCoordinate,
        responses_channel: &mut ChunkLoadResponseChannel,
    ) {
        let coordinates_to_load = compute_chunks_to_load_after_player_current_chunk_change(
            self.current_chunk,
            new_player_chunk_coordinate,
        );

        self.current_chunk = new_player_chunk_coordinate;

        // TODO: Consider invalidating the old chunks

        for coordinate in coordinates_to_load {
            let chunk = self.chunk_source.get_chunk_at(coordinate);
            let response = ChunkLoadResult { coordinate, chunk };
            let _ = responses_channel.send(response);
        }
    }

    #[inline(always)]
    fn process_initial_player_coordinate(
        &mut self,
        initial_coordinate: ChunkCoordinate,
        responses_channel: &mut ChunkLoadResponseChannel,
    ) {
        let mut load_chunk = |i, j| {
            let coordinate = ChunkCoordinate { i, j };
            let chunk = self.chunk_source.get_chunk_at(coordinate);
            let response = ChunkLoadResult { coordinate, chunk };
            let _ = responses_channel.send(response);
        };

        load_chunk(initial_coordinate.i, initial_coordinate.j);

        if constants::RENDER_DISTANCE_CHUNKS > 0 {
            for d in 1..=(constants::RENDER_DISTANCE_CHUNKS as i32) {
                let i_left = initial_coordinate.i - d;
                let i_right = initial_coordinate.i + d;
                let j_top = initial_coordinate.j + d;
                let j_bottom = initial_coordinate.j - d;

                // Top
                for i in i_left..i_right {
                    load_chunk(i, j_top);
                }

                // Bottom
                for i in (i_left + 1)..=i_right {
                    load_chunk(i, j_bottom);
                }

                // Left
                for j in j_bottom..j_top {
                    load_chunk(i_left, j);
                }

                // Right
                for j in (j_bottom + 1)..=j_top {
                    load_chunk(i_right, j);
                }
            }
        }
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
    use constants::*;
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
}
