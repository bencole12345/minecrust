use na::{Matrix4, Point3, Translation3, Vector3};

use crate::engine::lighting::{GlobalLight, PointLight};
use crate::engine::model::ModelData;
use crate::engine::skybox::Skybox;

/// An object present in a `Scene`
pub struct SceneObject {
    /// The position of the object in world coordinates
    pub position: Point3<f32>,

    /// The orientation of the object in the world
    pub orientation: Vector3<f32>,

    /// The scale of the object relative to the `ModelData`
    pub scale: f32,

    /// The model data for this object
    pub model_data: ModelData,
}

// A scene that can be rendered
pub struct Scene {
    /// The objects in the scene
    pub objects: Vec<SceneObject>,

    /// The point lights in the scene
    pub point_lights: Vec<PointLight>,

    /// The scene's global illuminant
    pub global_light: GlobalLight,

    /// The scene's skybox
    pub skybox: Skybox,
}

impl SceneObject {
    /// Compute the model matrix for this scene object
    pub fn model_matrix(&self) -> Matrix4<f32> {
        Translation3::from(self.position).to_homogeneous()
    }
}
