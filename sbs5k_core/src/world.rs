use nalgebra::Point3;

use sbs5k_messages;

/// The position of an object in the world
pub type Position = Point3<f32>;

/// The orientation of an object in the world
pub use sbs5k_messages::world::Orientation;

/// The position of an entity in the world
#[derive(Debug)]
pub struct EntityPosition {
    /// The entity's position in world space
    pub position: Position,

    /// The orientation of the entity in the world
    pub orientation: Orientation,
}
