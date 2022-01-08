use na::{Point3, Vector3};

use crate::engine::{lighting, scene, skybox};
use crate::world;

pub fn build_example_scene() -> scene::Scene {
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

    let skybox = skybox::Skybox::new();

    scene::Scene {
        objects,
        point_lights,
        global_light,
        skybox,
    }
}
