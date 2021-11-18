use glfw::{Context, FlushedMessages, WindowEvent};

use crate::engine::driver::DriverHost;

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

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // Load OpenGL functions
        // TODO: Move this into some kind of thread-safe singleton class
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window {
            glfw_instance: glfw,
            glfw_window: window,
            glfw_events: events,
        }
    }
}

impl DriverHost for Window {
    fn should_continue(&self) -> bool {
        !self.glfw_window.should_close()
    }

    fn poll_events(&mut self) -> FlushedMessages<'_, (f64, WindowEvent)> {
        self.glfw_instance.poll_events();
        glfw::flush_messages(&self.glfw_events)
    }

    fn swap_buffers(&mut self) {
        self.glfw_window.swap_buffers();
    }
}
