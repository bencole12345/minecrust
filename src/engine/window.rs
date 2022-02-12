use glfw::{Action, Context};

use crate::engine::events::{Event, EventSource};
use crate::engine::inputs::Key;
use crate::engine::rendering::RenderingContext;

/// A window that will contain the game
pub struct Window {
    glfw_instance: glfw::Glfw,
    glfw_window: glfw::Window,
    glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // Enable 4x MSAA
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // Load OpenGL functions
        // TODO: Move this into some kind of thread-safe, lazy-loaded singleton class
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window {
            glfw_instance: glfw,
            glfw_window: window,
            glfw_events: events,
        }
    }

    pub fn alive(&self) -> bool {
        !self.glfw_window.should_close()
    }
}

impl RenderingContext for Window {
    fn swap_buffers(&mut self) {
        self.glfw_window.swap_buffers();
    }
}

impl EventSource for Window {
    fn poll_events(&mut self) -> Vec<Event> {
        self.glfw_instance.poll_events();
        glfw::flush_messages(&self.glfw_events)
            .filter_map(|(_, event)| {
                match event {
                    glfw::WindowEvent::Key(glfw_key, _, Action::Press, _) => {
                        let key = Key::from_glfw_key(glfw_key);
                        match key {
                            Some(k) => Some(Event::KeyPress(k)),
                            None => None,
                        }
                    }

                    glfw::WindowEvent::Key(glfw_key, _, Action::Release, _) => {
                        let key = Key::from_glfw_key(glfw_key);
                        match key {
                            Some(k) => Some(Event::KeyRelease(k)),
                            None => None,
                        }
                    }

                    // TODO: Add mouse movement
                    _ => None,
                }
            })
            .collect()
    }
}
