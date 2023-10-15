use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use sbs5k_core::chunk;
use sbs5k_engine as engine;

use crate::args;
use crate::controls;
use crate::debug;
use crate::event;
use crate::initialisation;
use crate::loading;
use crate::state;

const TITLE: &str = "Super Block Simulator 5000";
const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

/// The main entrypoint for the game client
pub(crate) struct Driver {
    running: Rc<Cell<bool>>,
    config: Arc<args::Args>,
    window: engine::Window,
    chunk_load_request_tx: mpsc::Sender<loading::ChunkLoadRequest>,
    chunk_load_result_rx: mpsc::Receiver<loading::ChunkLoadResult>,
    _chunk_loader_thread_handle: thread::JoinHandle<()>,
    controls: Rc<RefCell<controls::ControlsHandler>>,
    skybox: engine::Skybox,
    renderer: engine::Renderer,
    scene_lighting: engine::lighting::SceneLighting,
    fog_parameters: engine::FogParameters,
    mesh_generator: loading::MeshGenerator,
    event_queue: event::EventQueue,
    state: Box<state::ClientState>,
}

impl Driver {
    pub(crate) fn new(
        config: Arc<args::Args>,
        chunk_source: Box<dyn chunk::ChunkSource + Send>,
    ) -> Self {
        let state = Box::new(state::ClientState::new(&config));

        // We have two flags: the Arc for communicating to the chunk generation thread that it's time to stop, and an
        // Rc for use in the main thread without any cache thrashing
        let live_flag = state.is_live.clone();
        let running_flag = Rc::new(Cell::new(true));

        // Set up the asynchronous chunk loading system
        let (chunk_load_request_tx, chunk_load_request_rx) = mpsc::channel();
        let (chunk_load_result_tx, chunk_load_result_rx) = mpsc::channel();

        // Need to copy the config ready to move it to the other thread
        let config_copy = config.clone();

        let chunk_loader_thread_handle = thread::spawn(move || {
            let mut chunk_loader = loading::ChunkLoader::new(chunk_source, config_copy, live_flag);
            chunk_loader.generate_chunks_until_stop(chunk_load_request_rx, chunk_load_result_tx);
        });

        let fog_parameters = initialisation::make_fog_parameters(&config);

        let mut event_queue = event::EventQueue::new();

        let controls = Rc::new(RefCell::new(controls::ControlsHandler::new(
            event_queue.get_submitter(),
        )));
        let movement_applier = Rc::new(RefCell::new(controls::MovementApplier::new(
            state.player_position.clone(),
        )));

        let stopper = Rc::new(RefCell::new(Stopper {
            flag: running_flag.clone(),
        }));

        event_queue.add_listener(movement_applier);
        event_queue.add_listener(stopper);

        Driver {
            running: running_flag,
            config,
            window: engine::Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, TITLE),
            chunk_load_request_tx,
            chunk_load_result_rx,
            _chunk_loader_thread_handle: chunk_loader_thread_handle,
            controls,
            skybox: engine::Skybox::new(),
            renderer: engine::Renderer::new(),
            scene_lighting: initialisation::make_scene_lighting(),
            fog_parameters,
            mesh_generator: loading::MeshGenerator::new(),
            event_queue,
            state,
        }
    }

    /// Run the game to completion
    ///
    /// This method contains the game's main loop.
    pub(crate) fn run_game(&mut self) {
        let mut time_tracker = engine::TimeTracker::new();

        // TODO: Move this to the event queue system
        // Set up an initial "previous" value. This value is used for working out the set of
        // chunks that need to be loaded whenever the player crosses a boundary
        let mut prev_player_chunk = chunk::ChunkCoordinate::from_player_position(
            self.state.player_position.borrow().location,
        );

        self.chunk_load_request_tx
            .send(loading::ChunkLoadRequest::InitialLoad(prev_player_chunk))
            .unwrap();

        if self.config.verbose {
            println!("Starting main loop");
        }

        while self.running.get() && self.window.alive() {
            // TODO: Move this to the event queue system
            // Respond to any chunk creation events
            while let Ok(loaded_chunk) = self.chunk_load_result_rx.try_recv() {
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

            time_tracker.tick();

            // First dispatch any window-triggered events to the event loop
            self.controls
                .borrow_mut()
                .pump_window_events(&mut self.window, time_tracker.dt() as f32);

            // Then perform the main event queue dispatch, which may include motion events triggered by the above
            self.event_queue.dispatch_all_events();

            let (player_current_location, player_current_orientation) = {
                let player_pos = self.state.player_position.borrow();
                (player_pos.location, player_pos.orientation)
            };

            let camera_pos = engine::CameraPosition {
                position: player_current_location,
                yaw: player_current_orientation.yaw,
                pitch: player_current_orientation.pitch,
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

            if self.config.is_in_debug_mode() {
                debug::print_debug_output(&self.state, time_tracker.dt(), &self.config);
            }

            let current_player_chunk =
                chunk::ChunkCoordinate::from_player_position(player_current_location);
            if current_player_chunk != prev_player_chunk {
                self.chunk_load_request_tx
                    .send(loading::ChunkLoadRequest::ChunkChangeLoad(
                        current_player_chunk,
                    ))
                    .unwrap();
            }
            prev_player_chunk = current_player_chunk;
        }

        // There are two possible states here: either the worker thread might be in the middle of a
        // (possibly large) chunk generation procedure, or it could be blocked waiting for us to
        // send it another message. To prevent deadlocks during shutdown, we need to *both* set the
        // "live" flag to the "dead" state and also send the worker thread a "stop" message, to
        // cover the busy and blocked cases and exit as quickly as possible.
        self.state.mark_dead();
        self.chunk_load_request_tx
            .send(loading::ChunkLoadRequest::Stop)
            .unwrap();
    }
}

struct Stopper {
    flag: Rc<Cell<bool>>,
}

impl event::EventListener for Stopper {
    fn on_event(&mut self, event: &event::Event) {
        if let event::Event::EndGame = event {
            self.flag.replace(false);
        }
    }
}
