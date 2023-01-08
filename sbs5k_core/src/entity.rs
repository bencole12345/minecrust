use nalgebra::Point3;

/// The position of an entity in the world
#[derive(Debug)]
pub struct EntityPosition {
    /// The entity's position in world space
    pub position: Point3<f32>,

    /// The elevation of the entity above the XZ plane, in radians
    pub pitch: f32,

    /// The rotation of the entity clockwise around the Y axis from the +Z ray
    pub yaw: f32,

    /// The rotation of the entity clockwise around a line through the XZ axis
    pub roll: f32,
}
