extern crate glfw;

mod engine;

use engine::window;

fn main() {
    let width = 1920;
    let height = 1080;
    let title = "Hello";

    let mut window = window::create_window(width, height, &title);
    window.main_loop();
}
