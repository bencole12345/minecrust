use serde::{Deserialize, Serialize};

/// A location within the 3D world
pub type Location = na::Point3<f32>;

/// A change in location
pub type LocationDelta = na::Vector3<f32>;

/// An Euler-angle-based orientation in the world
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
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
pub struct EntityPosition {
    pub location: Location,
    pub orientation: Orientation,
}
