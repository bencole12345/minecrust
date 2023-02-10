use std::f32::consts::PI;

use glm::{cos, fmod, sin};
use nalgebra::Vector3;

use sbs5k_core::world;

/// An object that can be translated relative to its current orientation
pub trait Translatable {
    /// Move `distance` units forwards
    fn translate_forwards(&mut self, distance: f32);

    /// Move `distance` units backwards
    fn translate_backwards(&mut self, distance: f32);

    /// Move `distance` units to the left
    fn translate_left(&mut self, distance: f32);

    /// Move `distance` units to the right
    fn translate_right(&mut self, distance: f32);

    /// Move `distance` units upwards
    fn translate_up(&mut self, distance: f32);

    /// Move `distance` units downwards
    fn translate_down(&mut self, distance: f32);
}

/// An object with adjustable pitch and yaw
pub trait Rotatable {
    /// Increase the object's pitch by `angle` radians
    fn adjust_pitch(&mut self, angle: f32);

    /// Increase the object's yaw by `angle` radians
    fn adjust_yaw(&mut self, angle: f32);
}

impl Translatable for world::EntityPosition {
    fn translate_forwards(&mut self, distance: f32) {
        let x = -sin(self.orientation.yaw);
        let y = 0.0;
        let z = cos(self.orientation.yaw);
        let direction = Vector3::new(x, y, z);
        self.position += direction * distance;
    }

    fn translate_backwards(&mut self, distance: f32) {
        let x = sin(self.orientation.yaw);
        let y = 0.0;
        let z = -cos(self.orientation.yaw);
        let direction = Vector3::new(x, y, z);
        self.position += direction * distance;
    }

    fn translate_left(&mut self, distance: f32) {
        let x = cos(self.orientation.yaw);
        let y: f32 = 0.0;
        let z = sin(self.orientation.yaw);
        let direction = Vector3::new(x, y, z);
        self.position += direction * distance;
    }

    fn translate_right(&mut self, distance: f32) {
        let x = -cos(self.orientation.yaw);
        let y = 0.0;
        let z = -sin(self.orientation.yaw);
        let direction = Vector3::new(x, y, z);
        self.position += direction * distance;
    }

    fn translate_up(&mut self, distance: f32) {
        let direction = Vector3::new(0.0, 1.0, 0.0);
        self.position += direction * distance;
    }

    fn translate_down(&mut self, distance: f32) {
        let direction = Vector3::new(0.0, -1.0, 0.0);
        self.position += direction * distance;
    }
}

// TODO: Only implement for orientation?
impl Rotatable for world::EntityPosition {
    fn adjust_pitch(&mut self, angle: f32) {
        self.orientation.pitch = glm::clamp(self.orientation.pitch + angle, -PI * 0.49, PI * 0.49);
    }

    fn adjust_yaw(&mut self, angle: f32) {
        self.orientation.yaw = fmod(self.orientation.yaw + angle, 2.0 * PI);
    }
}