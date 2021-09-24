extern crate glfw;

use glfw::{Action, Context, Key};

pub struct Window {
    pub glfw_instance: glfw::Glfw,
    pub glfw_window: glfw::Window,
    pub glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>
}

pub fn create_window(width: u32, height: u32, title: &str) -> Window {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    window.set_key_polling(true);
    window.make_current();

    Window {
        glfw_instance: glfw,
        glfw_window: window,
        glfw_events: events
    }
}

impl Window {
    pub fn main_loop(&mut self) {
        while !self.glfw_window.should_close() {
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
        }
    }
}

fn handle_event(_window_event: glfw::WindowEvent) {
    // TODO: Dispatch to a special handler
}
