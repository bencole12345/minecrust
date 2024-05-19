use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

use sbs5k_core::{chunk, geometry};
use sbs5k_engine as engine;

use crate::args;
use crate::backend_connection::BackendConnection;
use crate::controls;
use crate::debug;
use crate::event;
use crate::event::Event;
use crate::initialisation;
use crate::loading;
use crate::state;
use crate::updatable::Updatable;

const TITLE: &str = "Super Block Simulator 5000";
const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

/// The main entrypoint for the game client
pub(crate) struct Driver {
    running: Rc<Cell<bool>>,
    config: Arc<args::Args>,
    controls: Rc<RefCell<controls::ControlsHandler>>,
    skybox: engine::Skybox,
    renderer: engine::Renderer,
    scene_lighting: engine::lighting::SceneLighting,
    fog_parameters: engine::FogParameters,
    event_queue: event::EventQueue,
    event_submitter: event::EventSubmitter,
    state: Box<state::ClientState>,
    time_tracker: engine::TimeTracker,
    window: engine::Window,
}

impl Driver {
    pub(crate) fn new(
        config: Arc<args::Args>,
        chunk_source: Box<dyn chunk::ChunkSource + Send>,
    ) -> Self {
        let state = Box::new(state::ClientState::new(&config));

        // We have two "running" flags, one in `state` and the other here. `state`'s flag is
        // intended for communicating to background threads (e.g. the chunk loading worker thread)
        // that it's time to stop working, whereas `running_flag` is intended to be checked every
        // frame by the driver. The reason for this separation is to avoid an atomic read operation
        // on every tick.
        let running_flag = Rc::new(Cell::new(true));

        let window = engine::Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, TITLE);

        let fog_parameters = initialisation::make_fog_parameters(&config);

        let mut event_queue = event::EventQueue::new(state.is_live.clone());
        let event_submitter = event_queue.get_submitter();

        let stopper = Rc::new(RefCell::new(Stopper {
            flag: running_flag.clone(),
        }));
        let chunk_loader = Rc::new(RefCell::new(loading::ChunkLoader::new(
            chunk_source,
            event_queue.get_submitter(),
            config.clone(),
            state.is_live.clone(),
        )));
        let chunk_mesh_builder = Rc::new(RefCell::new(ChunkMeshCreator {
            mesh_generator: loading::MeshGenerator::new(),
            chunks_state: state.chunks_state.clone(),
        }));

        event_queue.add_listener(stopper);
        event_queue.add_listener(chunk_loader);
        event_queue.add_listener(chunk_mesh_builder);

        let controls = Rc::new(RefCell::new(controls::ControlsHandler::new(
            state.player_position.clone(),
            event_queue.get_submitter(),
        )));

        // Set up connection to the backend server, if connection details were provided
        if let Some(addr) = config.server {
            // TODO: Handle connection failure better
            let username = config
                .username
                .clone()
                .expect("--server without --username is invalid");
            let backend_updater = Rc::new(RefCell::new(
                BackendConnection::new(addr, username, event_queue.get_submitter())
                    .expect("Failed to create backend connection"),
            ));
            event_queue.add_listener(backend_updater);
        }

        Driver {
            running: running_flag,
            config,
            controls,
            skybox: engine::Skybox::new(),
            renderer: engine::Renderer::new(),
            scene_lighting: initialisation::make_scene_lighting(),
            fog_parameters,
            event_queue,
            event_submitter,
            state,
            time_tracker: engine::TimeTracker::new(),
            window,
        }
    }

    /// Run the game to completion
    ///
    /// This method contains the game's main loop.
    // TODO: Would be groovy if the engine could own the main loop and call into a user-provided
    // function each frame. It could even own handling inputs and dispatching events, e.g. having
    // the client provide an onInput callback
    pub(crate) fn run_game(&mut self) {
        // Set up an initial "previous" value. This value is used for working out the set of
        // chunks that need to be loaded whenever the player crosses a boundary
        let mut prev_player_chunk = chunk::ChunkCoordinate::from_player_position(
            self.state.player_position.borrow().location,
        );

        // TODO: Switch to a proper logging system
        if self.config.verbose {
            println!("Starting main loop");
        }

        while self.running.get() && self.window.alive() {
            self.time_tracker.tick();
            {
                self.controls
                    .borrow_mut()
                    .pump_window_events(&mut self.window);
            }

            self.update_all_components();

            // Then perform the main event queue dispatch, which may include motion events triggered by the above
            self.event_queue.dispatch_all_events();

            let (player_current_location, player_current_orientation) = {
                let player_pos = self.state.player_position.borrow();
                (player_pos.location, player_pos.orientation)
            };

            self.render(player_current_location, player_current_orientation);

            if self.config.is_in_debug_mode() {
                debug::print_debug_output(&self.state, self.time_tracker.dt(), &self.config);
            }

            let current_player_chunk =
                chunk::ChunkCoordinate::from_player_position(player_current_location);
            if current_player_chunk != prev_player_chunk {
                self.event_submitter
                    .submit_event(Event::PlayerEnteredNewChunk(current_player_chunk));
            }
            prev_player_chunk = current_player_chunk;
        }

        // There are two possible states here: either the worker thread might be in the middle of a
        // (possibly large) chunk generation procedure, or it could be blocked waiting for us to
        // send it another message. To prevent deadlocks during shutdown, we need to *both* set the
        // "live" flag to the "dead" state and also send the worker thread a "stop" message, to
        // cover the busy and blocked cases and exit as quickly as possible.
        self.state.mark_dead();
    }

    fn update_all_components(&mut self) {
        // TODO: Make this a set of Updatables
        self.controls
            .borrow_mut()
            .update(self.time_tracker.dt() as f32);
    }

    fn render(
        &mut self,
        player_location: geometry::Location,
        player_orientation: geometry::Orientation,
    ) {
        let chunks_state = self.state.chunks_state.borrow_mut();
        let camera_pos = engine::CameraPosition {
            position: player_location,
            yaw: player_orientation.yaw,
            pitch: player_orientation.pitch,
        };
        self.renderer
            .do_render_pass(&mut self.window, &|render_target| {
                // Render each chunk
                render_target.render_objects(
                    &chunks_state.renderable_chunks(),
                    &self.scene_lighting,
                    &camera_pos,
                    &self.fog_parameters,
                );

                // Render the skybox
                render_target.render_skybox(&self.skybox, &camera_pos);
            });
    }
}

struct Stopper {
    flag: Rc<Cell<bool>>,
}

impl event::EventListener for Stopper {
    fn on_event(&mut self, event: &Event) {
        if let Event::EndGame = event {
            self.flag.replace(false);
        }
    }
}

struct ChunkMeshCreator {
    mesh_generator: loading::MeshGenerator,
    chunks_state: Rc<RefCell<state::ChunksState>>,
}

impl event::EventListener for ChunkMeshCreator {
    fn on_event(&mut self, event: &Event) {
        if let Event::ChunkLoaded(result) = event {
            let mut chunks_state = self.chunks_state.borrow_mut();
            let mesh = self
                .mesh_generator
                .chunk_to_scene_object(result.chunk.as_ref(), result.coordinate);
            chunks_state.set_chunk(result.coordinate, Some(*result.chunk));
            chunks_state.set_chunk_mesh(result.coordinate, Some(mesh));
        }
    }
}
