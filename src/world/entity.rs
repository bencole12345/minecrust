use na::Point3;

/// The position of an entity in the world
#[derive(Debug)]
pub struct EntityPosition {
    pub position: Point3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}
