use glm::{cos, fmod, sin};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// A location within the 3D world
pub type Location = na::Point3<f32>;

/// A change in location
pub type LocationDelta = na::Vector3<f32>;

/// An Euler-angle-based orientation in the world
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Orientation {
    /// The elevation of the entity above the XZ plane, in radians
    pub pitch: f32,

    /// The rotation of the entity clockwise around the Y axis from the +Z ray
    pub yaw: f32,

    /// The rotation of the entity clockwise around a line through the XZ axis
    pub roll: f32,
}

/// A change in orientation
pub struct OrientationDelta {
    pub delta_pitch: f32,
    pub delta_yaw: f32,
}

/// The location and orientation of an entity in the game world.
///
/// Useful as a combined type, especially for implementing traits like `Translatable`.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct EntityPosition {
    pub location: Location,
    pub orientation: Orientation,
}

impl EntityPosition {
    pub fn translate_forwards(&mut self, distance: f32) {
        let x = -sin(self.orientation.yaw);
        let y = 0.0;
        let z = cos(self.orientation.yaw);
        let direction = na::Vector3::new(x, y, z);
        self.location += direction * distance;
    }

    pub fn translate_right(&mut self, distance: f32) {
        let x = -cos(self.orientation.yaw);
        let y = 0.0;
        let z = -sin(self.orientation.yaw);
        let direction = na::Vector3::new(x, y, z);
        self.location += direction * distance;
    }

    pub fn translate_up(&mut self, distance: f32) {
        let direction = na::Vector3::new(0.0, 1.0, 0.0);
        self.location += direction * distance;
    }

    pub fn adjust_pitch(&mut self, angle: f32) {
        self.orientation.pitch = glm::clamp(self.orientation.pitch + angle, -PI * 0.49, PI * 0.49);
    }

    pub fn adjust_yaw(&mut self, angle: f32) {
        self.orientation.yaw = fmod(self.orientation.yaw + angle, 2.0 * PI);
    }
}

impl Default for EntityPosition {
    fn default() -> Self {
        Self {
            location: Location::new(0.0, 0.0, 0.0),
            orientation: Orientation {
                pitch: 0.0,
                yaw: 0.0,
                roll: 0.0,
            },
        }
    }
}
