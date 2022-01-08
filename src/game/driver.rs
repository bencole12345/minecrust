use super::controls::Controls;
use super::example;
use crate::engine::{camera, rendering, scene, time, window};

const TITLE: &str = "MineCrust";
const INITIAL_WIDTH: u32 = 1280;
const INITIAL_HEIGHT: u32 = 720;

/// The main entrypoint for the game client
pub struct Driver {
    window: window::Window,
    scene: scene::Scene,
    camera: camera::Camera,
    renderer: rendering::Renderer,
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            window: window::Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, TITLE),
            scene: example::build_example_scene(),
            camera: camera::Camera::default(),
            renderer: rendering::Renderer::new(),
        }
    }

    // TODO: Investigate interior mutability pattern to get rid of mutable references in here
    pub fn run_game(&mut self) {
        let mut time_tracker = time::TimeTracker::new();
        let mut controls = Controls::new();

        self.renderer.setup();

        'main_loop: loop {
            // Render scene to window
            self.renderer
                .render_scene(&self.scene, &self.camera, &mut self.window);

            time_tracker.tick();

            // Handle all events that happened since the last frame
            controls.consume_events(&mut self.window);

            // Adjust the camera position
            controls.update_camera(&mut self.camera, time_tracker.dt());

            if !self.window.alive() || controls.close_has_been_pressed() {
                break 'main_loop;
            }
        }
    }
}
