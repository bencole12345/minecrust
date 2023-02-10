use std::sync::{mpsc, Arc, RwLock};

use sbs5k_core::chunk::{Chunk, ChunkIndex, ChunkSource};

use crate::Args;

pub(crate) enum ChunkLoadRequest {
    InitialLoad(ChunkIndex),
    ChunkChangeLoad(ChunkIndex),
    Stop,
}

pub(crate) struct ChunkLoadResult {
    pub chunk: Box<Chunk>,
    pub coordinate: ChunkIndex,
}

pub(crate) struct ChunkLoader {
    chunk_source: Box<dyn ChunkSource + Send>,
    current_chunk: ChunkIndex,
    config: Args,
    is_live_flag: Arc<RwLock<bool>>,
}

impl ChunkLoader {
    pub(crate) fn new(
        chunk_source: Box<dyn ChunkSource + Send>,
        config: Args,
        is_live_flag: Arc<RwLock<bool>>,
    ) -> Self {
        Self {
            chunk_source,
            current_chunk: Default::default(),
            config,
            is_live_flag,
        }
    }

    pub(crate) fn generate_chunks_until_stop(
        &mut self,
        requests_receiver: mpsc::Receiver<ChunkLoadRequest>,
        mut responses_sender: mpsc::Sender<ChunkLoadResult>,
    ) {
        'generate_loop: for chunk_load_request in requests_receiver {
            // Stop early if the game is no longer live
            if !*self.is_live_flag.read().unwrap() {
                break 'generate_loop;
            }

            // Process the latest request
            match chunk_load_request {
                ChunkLoadRequest::InitialLoad(coordinate) => {
                    self.process_initial_load(coordinate, &mut responses_sender);
                }

                ChunkLoadRequest::ChunkChangeLoad(coordinate) => {
                    self.process_chunk_change_load(coordinate, &mut responses_sender);
                }

                ChunkLoadRequest::Stop => {
                    break 'generate_loop;
                }
            }
        }
    }

    #[inline(always)]
    fn process_initial_load(
        &mut self,
        initial_coordinate: ChunkIndex,
        responses_channel: &mut mpsc::Sender<ChunkLoadResult>,
    ) {
        let live_flag = self.is_live_flag.clone();

        let mut load_chunk = |i, j| {
            let coordinate = ChunkIndex { i, j };
            let chunk = self.chunk_source.get_chunk_at(coordinate);
            let response = ChunkLoadResult { coordinate, chunk };
            let _ = responses_channel.send(response);
        };

        load_chunk(initial_coordinate.i, initial_coordinate.j);

        self.current_chunk = initial_coordinate;

        // Load the remaining initial chunks in a spiral shape around the player so that
        // the chunks closest to the player get loaded first
        if self.config.render_distance > 0 {
            for d in 1..=(self.config.render_distance as i32) {
                if !*live_flag.read().unwrap() {
                    return;
                }

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

    #[inline(always)]
    fn process_chunk_change_load(
        &mut self,
        new_coordinate: ChunkIndex,
        responses_channel: &mut mpsc::Sender<ChunkLoadResult>,
    ) {
        let coordinates_to_load = compute_chunks_to_load_after_player_current_chunk_change(
            self.current_chunk,
            new_coordinate,
            self.config.render_distance,
        );

        self.current_chunk = new_coordinate;

        // TODO: Consider invalidating the old chunks

        for coordinate in coordinates_to_load {
            let chunk = self.chunk_source.get_chunk_at(coordinate);
            let response = ChunkLoadResult { coordinate, chunk };
            let _ = responses_channel.send(response);
        }
    }
}

/// Compute the set of chunk coordinates that must be loaded (and the ones in their places dropped)
/// if the player moved from `old_chunk_coord` to `new_chunk_coord`
fn compute_chunks_to_load_after_player_current_chunk_change(
    old_chunk_coord: ChunkIndex,
    new_chunk_coord: ChunkIndex,
    render_distance: u32,
) -> Vec<ChunkIndex> {
    let range_before = renderable_chunk_indices_range(old_chunk_coord, render_distance);
    let range_after = renderable_chunk_indices_range(new_chunk_coord, render_distance);
    let (min_chunk, max_chunk) = range_after;

    let mut coords = vec![];
    for i in min_chunk.i..=max_chunk.i {
        for j in min_chunk.j..=max_chunk.j {
            let coord = ChunkIndex { i, j };
            if !chunk_coordinate_is_in_range(coord, range_before) {
                coords.push(coord);
            }
        }
    }
    coords
}

/// Compute the range of chunks that should be renderable for a given player index
fn renderable_chunk_indices_range(
    current_chunk_coord: ChunkIndex,
    render_distance: u32,
) -> (ChunkIndex, ChunkIndex) {
    let min_i = current_chunk_coord.i - render_distance as i32;
    let min_j = current_chunk_coord.j - render_distance as i32;
    let max_i = current_chunk_coord.i + render_distance as i32;
    let max_j = current_chunk_coord.j + render_distance as i32;
    let min = ChunkIndex { i: min_i, j: min_j };
    let max = ChunkIndex { i: max_i, j: max_j };
    (min, max)
}

/// Determine whether a chunk coordinate lies within a given range
#[inline]
fn chunk_coordinate_is_in_range(index: ChunkIndex, range: (ChunkIndex, ChunkIndex)) -> bool {
    let (lower, upper) = range;
    let i_in_range = index.i >= lower.i && index.i <= upper.i;
    let j_in_range = index.j >= lower.j && index.j <= upper.j;
    i_in_range && j_in_range
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const RENDER_DISTANCE_CHUNKS: u32 = 10;

    #[rstest]
    #[case(ChunkIndex{i: 0, j: 0},
    ChunkIndex{i: - (RENDER_DISTANCE_CHUNKS as i32), j: - (RENDER_DISTANCE_CHUNKS as i32)},
    ChunkIndex{i: (RENDER_DISTANCE_CHUNKS as i32), j: (RENDER_DISTANCE_CHUNKS as i32)})]
    fn renderable_chunk_range_works(
        #[case] chunk_coord: ChunkIndex,
        #[case] expected_min: ChunkIndex,
        #[case] expected_max: ChunkIndex,
    ) {
        let (actual_min, actual_max) =
            renderable_chunk_indices_range(chunk_coord, RENDER_DISTANCE_CHUNKS);
        assert_eq!(expected_min, actual_min);
        assert_eq!(expected_max, actual_max);
    }
}
