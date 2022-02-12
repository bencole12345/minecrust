use std::f32::consts::PI;

use glm::{cos, fmod, sin};
use na::Vector3;

use crate::world::entity;

/// An object that can be translated and rotated
pub trait Movable {
    /// Move `distance` units forwards
    fn move_forwards(&mut self, distance: f32);

    /// Move `distance` units backwards
    fn move_backwards(&mut self, distance: f32);

    /// Move `distance` units to the left
    fn move_left(&mut self, distance: f32);

    /// Move `distance` units to the right
    fn move_right(&mut self, distance: f32);

    /// Move `distance` units upwards
    fn move_up(&mut self, distance: f32);

    /// Move `distance` units downwards
    fn move_down(&mut self, distance: f32);

    /// Rotate `angle` radians anticlockwise
    fn rotate_left(&mut self, angle: f32);

    /// Rotate `angle` radians clockwise
    fn rotate_right(&mut self, angle: f32);

    /// Rotate `angle` radians upwards
    fn rotate_up(&mut self, angle: f32);

    /// Rotate `angle` radians downwards
    fn rotate_down(&mut self, angle: f32);
}

impl Movable for entity::EntityPosition {
    fn move_forwards(&mut self, distance: f32) {
        let x = -sin(self.yaw);
        let y = 0.0;
        let z = cos(self.yaw);
        let direction = Vector3::new(x, y, z);
        self.position = self.position + direction * distance;
    }

    fn move_backwards(&mut self, distance: f32) {
        let x = sin(self.yaw);
        let y = 0.0;
        let z = -cos(self.yaw);
        let direction = Vector3::new(x, y, z);
        self.position = self.position + direction * distance;
    }

    fn move_left(&mut self, distance: f32) {
        let x = cos(self.yaw);
        let y: f32 = 0.0;
        let z = sin(self.yaw);
        let direction = Vector3::new(x, y, z);
        self.position = self.position + direction * distance;
    }

    fn move_right(&mut self, distance: f32) {
        let x = -cos(self.yaw);
        let y = 0.0;
        let z = -sin(self.yaw);
        let direction = Vector3::new(x, y, z);
        self.position = self.position + direction * distance;
    }

    fn move_up(&mut self, distance: f32) {
        let direction = Vector3::new(0.0, 1.0, 0.0);
        self.position = self.position + direction * distance;
    }

    fn move_down(&mut self, distance: f32) {
        let direction = Vector3::new(0.0, -1.0, 0.0);
        self.position = self.position + direction * distance;
    }

    // TODO: Investigate why these sometimes go negative
    // (I think fmod tries to preserve the sign of its input - the controls scheme still works totally fine)

    fn rotate_left(&mut self, angle: f32) {
        self.yaw = fmod(self.yaw - angle, 2.0 * PI);
    }

    fn rotate_right(&mut self, angle: f32) {
        self.yaw = fmod(self.yaw + angle, 2.0 * PI);
    }

    fn rotate_up(&mut self, angle: f32) {
        self.pitch = glm::clamp(self.pitch + angle, -PI * 0.95, PI * 0.95);
    }

    fn rotate_down(&mut self, angle: f32) {
        self.pitch = glm::clamp(self.pitch - angle, -PI * 0.95, PI * 0.95);
    }
}
