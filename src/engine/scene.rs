extern crate glm;
use glm::Vec3;

use super::lighting::{GlobalLight, PointLight};
use super::model::ModelData;

pub struct SceneObject {
    pub position: Vec3,
    pub orientation: Vec3,
    pub scale: f32,
    pub model_data: ModelData,
}

pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub point_lights: Vec<PointLight>,
    pub global_light: GlobalLight,
    // TODO: Add background/skybox (maybe use an enum to wrap the two)
}
