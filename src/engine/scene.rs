use na::{Matrix4, Point3, Translation3, Vector3};

use crate::engine::lighting::{GlobalLight, PointLight};
use crate::engine::model::ModelData;
use crate::engine::skybox::Skybox;

pub struct SceneObject {
    pub position: Point3<f32>,
    pub orientation: Vector3<f32>,
    pub scale: f32,
    pub model_data: ModelData,
}

pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub point_lights: Vec<PointLight>,
    pub global_light: GlobalLight,
    pub skybox: Skybox,
}

impl SceneObject {
    pub fn model_matrix(&self) -> Matrix4<f32> {
        Translation3::from(self.position).to_homogeneous()
    }
}
