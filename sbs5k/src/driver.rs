use std::sync::mpsc;
use std::thread;

use sbs5k_core::chunk;
use sbs5k_engine as engine;

use crate::args;
use crate::debug;
use crate::initialisation;
use crate::loading::ChunkLoadRequest;
use crate::state;
use crate::{controls, loading};

const TITLE: &str = "Super Block Simulator 5000";
const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

/// The main entrypoint for the game client
pub(crate) struct Driver {
    config: args::Args,
    window: engine::Window,
    skybox: engine::Skybox,
    renderer: engine::Renderer,
    scene_lighting: engine::lighting::SceneLighting,
    fog_parameters: engine::FogParameters,
    mesh_generator: loading::MeshGenerator,
    state: Box<state::ClientState>,
}

impl Driver {
    pub(crate) fn new(config: args::Args) -> Self {
        Driver {
            config,
            window: engine::Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, TITLE),
            skybox: engine::Skybox::new(),
            renderer: engine::Renderer::new(),
            scene_lighting: initialisation::make_scene_lighting(),
            fog_parameters: initialisation::make_fog_parameters(&config),
            mesh_generator: loading::MeshGenerator::new(),
            state: Box::new(state::ClientState::new(&config)),
        }
    }

    /// Run the game to completion
    ///
    /// This method contains the game's main loop.
    pub(crate) fn run_game(&mut self, chunk_source: Box<dyn chunk::ChunkSource + Send>) {
        let mut time_tracker = engine::TimeTracker::new();
        let mut controls = controls::ControlsHandler::new();

        // Set up the asynchronous chunk loading system
        let mut chunk_loader =
            loading::ChunkLoader::new(chunk_source, self.config, self.state.is_live.clone());
        let (chunk_load_request_tx, chunk_load_request_rx) = mpsc::channel();
        let (chunk_load_result_tx, chunk_load_result_rx) = mpsc::channel();
        let chunk_loader_thread_handle = thread::spawn(move || {
            chunk_loader.generate_chunks_until_stop(chunk_load_request_rx, chunk_load_result_tx);
        });

        let mut prev_player_chunk = chunk::ChunkIndex::from(self.state.player_position.position);

        chunk_load_request_tx
            .send(ChunkLoadRequest::InitialLoad(prev_player_chunk))
            .unwrap();

        if self.config.verbose {
            println!("Starting main loop");
        }

        'main_loop: loop {
            // Respond to any chunk creation events
            while let Ok(loaded_chunk) = chunk_load_result_rx.try_recv() {
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
                yaw: self.state.player_position.orientation.yaw,
                pitch: self.state.player_position.orientation.pitch,
            };

            // Render scene to window
            self.renderer
                .do_render_pass(&mut self.window, &|render_target| {
                    // Render each chunk
                    render_target.render_objects(
                        &self.state.chunks_state.renderable_chunks(),
                        &self.scene_lighting,
                        &camera_pos,
                        &self.fog_parameters,
                    );

                    // Render the skybox
                    render_target.render_skybox(&self.skybox, &camera_pos);
                });

            time_tracker.tick();

            if self.config.is_in_debug_mode() {
                debug::print_debug_output(&self.state, time_tracker.dt(), &self.config);
            }

            // Apply controls to update the player's position
            controls.consume_events(&mut self.window);
            controls.move_player(&mut self.state.player_position, time_tracker.dt());

            let current_player_chunk = chunk::ChunkIndex::from(self.state.player_position.position);
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

        // There are two possible states here: either the worker thread might be in the middle of a
        // (possibly large) chunk generation procedure, or it could be blocked waiting for us to
        // send it another message. To prevent deadlocks during shutdown, we need to *both* set the
        // "live" flag to the "dead" state and also send the worker thread a "stop" message, to
        // cover the busy and blocked cases and exit as quickly as possible.
        self.state.mark_dead();
        chunk_load_request_tx.send(ChunkLoadRequest::Stop).unwrap();
        chunk_loader_thread_handle.join().unwrap();
    }
}
