use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use sbs5k_core::chunk;

use crate::event::Event;
use crate::{args, event};

pub enum ChunkLoadRequest {
    InitialLoad,
    ChunkChangeLoad(chunk::ChunkCoordinate),
    Stop,
}

struct ChunkLoaderWorker {
    chunk_source: Box<dyn chunk::ChunkSource + Send>,
    requests_receiver: mpsc::Receiver<ChunkLoadRequest>,
    event_submitter: event::EventSubmitter,
    current_chunk_coordinate: chunk::ChunkCoordinate,
    config: Arc<args::Args>,
    is_live_flag: Arc<RwLock<bool>>,
}

impl ChunkLoaderWorker {
    pub(crate) fn new(
        chunk_source: Box<dyn chunk::ChunkSource + Send>,
        requests_receiver: mpsc::Receiver<ChunkLoadRequest>,
        event_submitter: event::EventSubmitter,
        config: Arc<args::Args>,
        is_live_flag: Arc<RwLock<bool>>,
    ) -> Self {
        Self {
            chunk_source,
            requests_receiver,
            event_submitter,
            current_chunk_coordinate: chunk::ChunkCoordinate::default(),
            config,
            is_live_flag,
        }
    }

    pub(crate) fn generate_chunks_until_stop(&mut self) {
        'generate_loop: loop {
            if let Ok(chunk_load_request) = self.requests_receiver.recv() {
                // Stop early if the game is no longer live
                if !*self.is_live_flag.read().unwrap() {
                    break 'generate_loop;
                }

                // Process the latest request
                match chunk_load_request {
                    ChunkLoadRequest::InitialLoad => {
                        self.process_initial_load();
                    }

                    ChunkLoadRequest::ChunkChangeLoad(coordinate) => {
                        self.process_chunk_change_load(coordinate);
                    }

                    ChunkLoadRequest::Stop => {
                        break 'generate_loop;
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn process_initial_load(&mut self) {
        let live_flag = self.is_live_flag.clone();

        let mut load_chunk = |i, j| {
            let coordinate = chunk::ChunkCoordinate { i, j };
            let chunk = self.chunk_source.get_chunk_at(coordinate);
            let result = chunk::ChunkLoadResult { coordinate, chunk };
            self.event_submitter
                .submit_event(Event::ChunkLoaded(result));
        };

        let initial_coordinate: chunk::ChunkCoordinate = Default::default();
        load_chunk(initial_coordinate.i, initial_coordinate.j);

        self.current_chunk_coordinate = initial_coordinate;

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
    fn process_chunk_change_load(&mut self, new_coordinate: chunk::ChunkCoordinate) {
        let coordinates_to_load = compute_chunks_to_load_after_player_current_chunk_change(
            self.current_chunk_coordinate,
            new_coordinate,
            self.config.render_distance,
        );

        self.current_chunk_coordinate = new_coordinate;

        // TODO: Consider invalidating the old chunks

        for coordinate in coordinates_to_load {
            let chunk = self.chunk_source.get_chunk_at(coordinate);
            let result = chunk::ChunkLoadResult { coordinate, chunk };
            self.event_submitter
                .submit_event(Event::ChunkLoaded(result));
        }
    }
}

pub(crate) struct ChunkLoader {
    chunk_load_request_tx: mpsc::Sender<ChunkLoadRequest>,
    chunk_loader_thread_handle: Option<thread::JoinHandle<()>>,
}

impl ChunkLoader {
    pub(crate) fn new(
        chunk_source: Box<dyn chunk::ChunkSource + Send>,
        event_submitter: event::EventSubmitter,
        config: Arc<args::Args>,
        is_live_flag: Arc<RwLock<bool>>,
    ) -> Self {
        // Communication from main thread -> background thread uses this channel. Communication from background
        // thread -> foreground thread will just use the event submitter.
        let (chunk_load_request_tx, chunk_load_request_rx) = mpsc::channel();

        let chunk_loader_thread_handle = Some(thread::spawn(move || {
            let mut chunk_loader = ChunkLoaderWorker::new(
                chunk_source,
                chunk_load_request_rx,
                event_submitter,
                config,
                is_live_flag,
            );
            chunk_loader.generate_chunks_until_stop();
        }));

        // Get the initial load started
        chunk_load_request_tx
            .send(ChunkLoadRequest::InitialLoad)
            .expect("Failed to send initial load request to queue");

        Self {
            chunk_load_request_tx,
            chunk_loader_thread_handle,
        }
    }
}

impl event::EventListener for ChunkLoader {
    fn on_event(&mut self, event: &Event) {
        if let Event::PlayerEnteredNewChunk(new_chunk_coord) = event {
            self.chunk_load_request_tx
                .send(ChunkLoadRequest::ChunkChangeLoad(*new_chunk_coord))
                .expect("Failed to add to queue");
        }
    }
}

impl Drop for ChunkLoader {
    fn drop(&mut self) {
        self.chunk_load_request_tx
            .send(ChunkLoadRequest::Stop)
            .expect("Failed to send stop request");
        if let Some(handle) = self.chunk_loader_thread_handle.take() {
            handle.join().expect("Failed to join chunk loading thread");
        }
    }
}

/// Compute the set of chunk coordinates that must be loaded (and the ones in their places dropped)
/// if the player moved from `old_chunk_coord` to `new_chunk_coord`
fn compute_chunks_to_load_after_player_current_chunk_change(
    old_chunk_coord: chunk::ChunkCoordinate,
    new_chunk_coord: chunk::ChunkCoordinate,
    render_distance: u32,
) -> Vec<chunk::ChunkCoordinate> {
    let range_before = renderable_chunk_indices_range(old_chunk_coord, render_distance);
    let range_after = renderable_chunk_indices_range(new_chunk_coord, render_distance);
    let (min_chunk, max_chunk) = range_after;

    let mut coords = vec![];
    for i in min_chunk.i..=max_chunk.i {
        for j in min_chunk.j..=max_chunk.j {
            let coord = chunk::ChunkCoordinate { i, j };
            if !chunk_coordinate_is_in_range(coord, range_before) {
                coords.push(coord);
            }
        }
    }
    coords
}

/// Compute the range of chunks that should be renderable for a given player index
fn renderable_chunk_indices_range(
    current_chunk_coord: chunk::ChunkCoordinate,
    render_distance: u32,
) -> (chunk::ChunkCoordinate, chunk::ChunkCoordinate) {
    let min_i = current_chunk_coord.i - render_distance as i32;
    let min_j = current_chunk_coord.j - render_distance as i32;
    let max_i = current_chunk_coord.i + render_distance as i32;
    let max_j = current_chunk_coord.j + render_distance as i32;
    let min = chunk::ChunkCoordinate { i: min_i, j: min_j };
    let max = chunk::ChunkCoordinate { i: max_i, j: max_j };
    (min, max)
}

/// Determine whether a chunk coordinate lies within a given range
#[inline]
fn chunk_coordinate_is_in_range(
    index: chunk::ChunkCoordinate,
    range: (chunk::ChunkCoordinate, chunk::ChunkCoordinate),
) -> bool {
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
    #[case(chunk::ChunkCoordinate{i: 0, j: 0},
    chunk::ChunkCoordinate{i: - (RENDER_DISTANCE_CHUNKS as i32), j: - (RENDER_DISTANCE_CHUNKS as i32)},
    chunk::ChunkCoordinate{i: (RENDER_DISTANCE_CHUNKS as i32), j: (RENDER_DISTANCE_CHUNKS as i32)})]
    fn renderable_chunk_range_works(
        #[case] chunk_coord: chunk::ChunkCoordinate,
        #[case] expected_min: chunk::ChunkCoordinate,
        #[case] expected_max: chunk::ChunkCoordinate,
    ) {
        let (actual_min, actual_max) =
            renderable_chunk_indices_range(chunk_coord, RENDER_DISTANCE_CHUNKS);
        assert_eq!(expected_min, actual_min);
        assert_eq!(expected_max, actual_max);
    }
}
