use na::Vector3;

use crate::engine::lighting;

/// Set up the initial lighting parameters for the scene
pub(crate) fn make_scene_lighting() -> lighting::SceneLighting {
    let point_lights = vec![];
    let global_light = lighting::GlobalLight {
        direction: Vector3::new(1.0, 1.2, 1.5).normalize(),
        colour: Vector3::new(1.0, 1.0, 1.0),
        intensity: 0.5,
    };

    lighting::SceneLighting {
        point_lights,
        global_light,
    }
}
