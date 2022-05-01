use crate::client::loading::{ChunkLoadResult, MeshGenerator};
use std::sync::mpsc;

use super::controls;
use super::debug;
use super::initialisation;
use super::loading::{ChunkLoadRequest, ChunkLoader};
use super::state;
use crate::engine;
use crate::world::chunk::{ChunkCoordinate, ChunkSource};

const TITLE: &'static str = "MineCrust";
const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

/// The main entrypoint for the game client
pub struct Driver {
    window: engine::Window,
    skybox: engine::Skybox,
    renderer: engine::Renderer,
    scene_lighting: engine::lighting::SceneLighting,
    fog_parameters: engine::FogParameters,
    mesh_generator: MeshGenerator,
    state: Box<state::ClientState>,
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            window: engine::Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, TITLE),
            skybox: engine::Skybox::new(),
            renderer: engine::Renderer::new(),
            scene_lighting: initialisation::make_scene_lighting(),
            fog_parameters: initialisation::make_fog_parameters(),
            mesh_generator: MeshGenerator::new(),
            state: Box::new(state::ClientState::new()),
        }
    }

    /// Run the game to completion
    ///
    /// This method contains the game's main loop.
    pub fn run_game(&mut self, chunk_source: Box<dyn ChunkSource + Send>) {
        let mut time_tracker = engine::TimeTracker::new();
        let mut controls = controls::ControlsHandler::new();

        self.renderer.setup();

        let (chunk_load_request_tx, chunk_load_response_rx) =
            setup_async_chunk_loading(chunk_source);

        // Do the initial chunk load
        let mut prev_player_chunk =
            ChunkCoordinate::from_player_position(self.state.player_position.position);
        chunk_load_request_tx
            .send(ChunkLoadRequest::InitialLoad(prev_player_chunk))
            .unwrap();

        println!("Starting main loop");

        'main_loop: loop {
            // Check if there are any meshes that need to be constructed this loop iteration
            while let Ok(loaded_chunk) = chunk_load_response_rx.try_recv() {
                let mesh = self
                    .mesh_generator
                    .chunk_to_scene_object(&loaded_chunk.chunk, loaded_chunk.coordinate);
                self.state
                    .chunks_state
                    .set_chunk(loaded_chunk.coordinate, Some(*loaded_chunk.chunk));
                self.state
                    .chunks_state
                    .set_chunk_mesh(loaded_chunk.coordinate, Some(mesh));
            }

            // Compute updated camera position
            let camera_pos = engine::CameraPosition {
                position: self.state.player_position.position,
                yaw: self.state.player_position.yaw,
                pitch: self.state.player_position.pitch,
            };

            // Render scene to window
            self.renderer.begin_render_pass(&self.window);

            {
                // TODO: Get rid of the lock here now that we no longer need the mutex
                self.renderer.render_objects(
                    &self.state.chunks_state.renderable_chunks(),
                    &self.scene_lighting,
                    &camera_pos,
                    &self.fog_parameters,
                );
            }
            self.renderer.render_skybox(&self.skybox, &camera_pos);
            self.renderer.complete_render_pass(&mut self.window);

            time_tracker.tick();

            if debug::DEBUGGING_ENABLED {
                debug::print_debug_output(&self.state, time_tracker.dt());
            }

            // Apply controls to update the player's position
            controls.consume_events(&mut self.window);
            controls.move_player(&mut self.state.player_position, time_tracker.dt());

            let current_player_chunk =
                ChunkCoordinate::from_player_position(self.state.player_position.position);
            if current_player_chunk != prev_player_chunk {
                chunk_load_request_tx
                    .send(ChunkLoadRequest::ChunkChangeLoad(current_player_chunk))
                    .unwrap();
            }
            prev_player_chunk = current_player_chunk;

            if !self.window.alive() || controls.close_has_been_pressed() {
                break 'main_loop;
            }
        }
    }
}

fn setup_async_chunk_loading(
    chunk_source: Box<dyn ChunkSource + Send>,
) -> (
    mpsc::Sender<ChunkLoadRequest>,
    mpsc::Receiver<ChunkLoadResult>,
) {
    let (chunk_load_request_tx, chunk_load_request_rx) = mpsc::channel();
    let (chunk_load_response_tx, chunk_load_response_rx) = mpsc::channel();
    let mut chunk_loader = ChunkLoader::new(chunk_source);
    let _chunk_loader_handle = std::thread::spawn(move || {
        chunk_loader.generate_chunks(chunk_load_request_rx, chunk_load_response_tx);
    });
    (chunk_load_request_tx, chunk_load_response_rx)
}
