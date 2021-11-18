use glfw::ffi::glfwGetTime;
use glfw::{Action, FlushedMessages, Key, WindowEvent};

use crate::engine::controls::MovementControlledCamera;
use crate::engine::rendering::Renderer;
use crate::engine::scene::Scene;

/// Drives the execution of the program
pub struct Driver {
    controlled_camera: MovementControlledCamera,
    renderer: Renderer,
    loaded_scene: Option<Scene>,
}

/// Interface for a host environment capable of containing a Driver
pub trait DriverHost {
    /// Informs the driver whether it should continue executing
    fn should_continue(&self) -> bool;

    /// Yields a list of events that have happened since the previous call
    fn poll_events(&mut self) -> FlushedMessages<'_, (f64, WindowEvent)>;

    /// Informs the host to swap the active framebuffer
    fn swap_buffers(&mut self);
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            controlled_camera: MovementControlledCamera::new(1.0, 1.0),
            renderer: Renderer::new(),
            loaded_scene: None,
        }
    }

    pub fn load_scene(&mut self, scene: Scene) {
        self.loaded_scene = Option::from(scene);
    }

    pub fn main_loop<HostType: DriverHost>(&mut self, host: &mut HostType) {
        if self.loaded_scene.is_none() {
            panic!("Attempted to run main_loop() without a loaded scene!");
        }

        let mut live = true;
        let mut previous_time = get_time();

        #[allow(unused_assignments)]
        let mut dt: f64 = 0.0;

        self.renderer.setup();

        while live && host.should_continue() {
            // Render the scene
            self.renderer.render_scene(
                self.loaded_scene.as_ref().unwrap(),
                self.controlled_camera.get_camera(),
            );
            host.swap_buffers();

            // Process all events that happened
            live = self.process_events(host);

            // Update dt
            let current_time = get_time();
            dt = current_time - previous_time;
            previous_time = current_time;

            // Update things
            self.controlled_camera.tick(dt);

            // TODO: Sleep to ensure desired framerate
        }
    }

    fn process_events<HostType: DriverHost>(&mut self, host: &mut HostType) -> bool {
        // TODO: Ultimately I'd like there to be an "InputEventProcessor" object or something
        // like that, with a registry of all bound keys and associated handler functions. It should
        // be possible to set a list of event processors in priority order that each match the
        // events they're interested in and fall back to the next stage in the chain otherwise.
        // Also, I'd like there to be an intermediate enum wrapping all inputs so that
        // ControlledCamera doesn't need to be aware of the glfw names.

        let mut still_live = true;
        'events_loop: for (_, event) in host.poll_events() {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    still_live = false;
                    break 'events_loop;
                }

                _ => self.controlled_camera.process_input_event(event),
            }
        }
        still_live
    }
}

/// Returns the current time
fn get_time() -> f64 {
    unsafe { glfwGetTime() as f64 }
}
