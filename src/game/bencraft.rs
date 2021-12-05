use na::{Point3, Vector3};

use crate::engine::{driver, lighting, scene, window};
use crate::world;

const INITIAL_WIDTH: u32 = 1280;
const INITIAL_HEIGHT: u32 = 720;

pub fn run_game() {
    let title = "MineCrust";
    let mut the_window = window::Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, &title);

    let objects = vec![scene::SceneObject {
        position: Point3::new(0.0, 0.0, -4.0),
        orientation: Vector3::new(0.0, 0.0, 0.0),
        scale: 1.0,
        model_data: world::cube::make_cube_model(),
    }];

    let point_lights: Vec<lighting::PointLight> = vec![
        lighting::PointLight {
            colour: Vector3::new(1.0, 0.8, 1.0),
            intensity: 5.0,
            position: Point3::new(1.5, 1.5, -3.0),
        },
        lighting::PointLight {
            colour: Vector3::new(0.3, 0.3, 1.0),
            intensity: 3.5,
            position: Point3::new(-1.0, 2.5, -3.0),
        },
        lighting::PointLight {
            colour: Vector3::new(1.0, 1.0, 1.0),
            intensity: 5.0,
            position: Point3::new(0.0, -3.0, -8.0),
        },
    ];

    let global_light = lighting::GlobalLight {
        direction: Vector3::new(0.577, 0.577, 0.577),
        colour: Vector3::new(1.0, 1.0, 1.0),
        intensity: 0.0,
    };

    let the_scene = scene::Scene {
        objects,
        point_lights,
        global_light,
    };

    let mut driver = driver::Driver::new();
    driver.load_scene(the_scene);

    driver.main_loop(&mut the_window);
}
