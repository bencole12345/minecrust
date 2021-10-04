mod engine;

extern crate glfw;
extern crate glm;
use glm::vec3;

use engine::{camera, lighting, scene, triangle, window};

fn main() {
    let width = 1920;
    let height = 1080;
    let title = "Hello";
    let mut window = window::create_window(width, height, &title);

    let the_triangle = triangle::make_triangle();
    let objects = vec![the_triangle];
    let point_lights: Vec<lighting::PointLight> = vec![];
    let global_light = lighting::GlobalLight {
        direction: vec3(0.0, 0.0, 0.0),
    };
    let mut the_scene = scene::Scene {
        objects,
        point_lights,
        global_light,
    };
    let mut the_camera = camera::Camera {
        position: vec3(0.0, 0.0, 0.0),
        orientation: vec3(0.0, 0.0, 0.0),
    };

    window.main_loop(&mut the_scene, &mut the_camera);
}
