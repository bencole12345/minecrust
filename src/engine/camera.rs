extern crate glm;
use glm::Vec3;

/// Encodes the position of the camera in the game world
pub struct Camera {
    pub position: Vec3,
    pub orientation: Vec3,
}

impl Camera {
    // TODO: Add methods like move_forwards(distance: f32)
}
