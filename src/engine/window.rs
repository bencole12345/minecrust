extern crate glfw;

use glfw::{Action, Context, Key};

use super::camera::Camera;
use super::rendering::Renderer;
use super::scene::Scene;

pub struct Window {
    glfw_instance: glfw::Glfw,
    glfw_window: glfw::Window,
    glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    renderer: Renderer,
}

pub fn create_window(width: u32, height: u32, title: &str) -> Window {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load OpenGL functions
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let renderer = Renderer::create();

    Window {
        glfw_instance: glfw,
        glfw_window: window,
        glfw_events: events,
        renderer: renderer,
    }
}

impl Window {
    pub fn main_loop(&mut self, scene: &mut Scene, camera: &mut Camera) {
        while !self.glfw_window.should_close() {
            // Handle events
            self.glfw_instance.poll_events();
            for (_, event) in glfw::flush_messages(&self.glfw_events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.glfw_window.set_should_close(true);
                    }

                    other_event => {
                        handle_event(other_event);
                    }
                }
            }

            // Render to the screen
            self.renderer.render_scene(scene, camera);
            self.glfw_window.swap_buffers();
        }
    }
}

fn handle_event(_window_event: glfw::WindowEvent) {
    // TODO: Dispatch to a special handler
}
