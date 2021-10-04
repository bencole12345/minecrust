extern crate glm;
use glm::Vec3;

pub struct PointLight {
    pub position: Vec3,
    pub colour: Vec3,
    pub intensity: f32,
}

pub struct GlobalLight {
    pub direction: Vec3,
}
