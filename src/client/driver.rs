use super::controls;
use super::initialisation;
use super::state;
use crate::engine;
use crate::world::chunk;

const TITLE: &'static str = "MineCrust";
const INITIAL_WIDTH: u32 = 1280;
const INITIAL_HEIGHT: u32 = 720;

const PRINT_FPS: bool = false;

/// The main entrypoint for the game client
pub struct Driver {
    window: engine::Window,
    scene_lighting: engine::lighting::SceneLighting,
    skybox: engine::Skybox,
    renderer: engine::Renderer,
    state: Box<state::ClientState>,
}

impl Driver {
    pub fn new(chunk_source: Box<dyn chunk::ChunkSource>) -> Self {
        Driver {
            window: engine::Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, TITLE),
            scene_lighting: initialisation::make_scene_lighting(),
            skybox: engine::Skybox::new(),
            renderer: engine::Renderer::new(),
            state: Box::new(state::ClientState::new(chunk_source)),
        }
    }

    /// Run the game to completion
    ///
    /// This method contains the game's main loop.
    pub fn run_game(&mut self) {
        let mut time_tracker = engine::TimeTracker::new();
        let mut controls = controls::ControlsHandler::new();

        self.renderer.setup();

        let mut prev_player_chunk_index =
            chunk::ChunkIndex::from_player_position(self.state.player_position.position);

        'main_loop: loop {
            // Compute updated camera position
            let camera_pos = engine::CameraPosition {
                position: self.state.player_position.position,
                yaw: self.state.player_position.yaw,
                pitch: self.state.player_position.pitch,
            };

            // Render scene to window
            self.renderer.begin_render_pass(&self.window);
            self.renderer.render_objects(
                self.state.renderable_chunks(),
                &self.scene_lighting,
                &camera_pos,
            );
            self.renderer.render_skybox(&self.skybox, &camera_pos);
            self.renderer.complete_render_pass(&mut self.window);

            time_tracker.tick();

            if PRINT_FPS {
                println!("FPS: {}", 1.0 / time_tracker.dt());
            }

            // Handle all events that happened since the last frame
            controls.consume_events(&mut self.window, &mut self.state, time_tracker.dt());

            let new_player_chunk_index =
                chunk::ChunkIndex::from_player_position(self.state.player_position.position);
            if new_player_chunk_index != prev_player_chunk_index {
                self.state
                    .notify_player_changed_chunk(new_player_chunk_index);
                prev_player_chunk_index = new_player_chunk_index;
            }

            if !self.window.alive() || controls.close_has_been_pressed() {
                break 'main_loop;
            }
        }
    }
}
