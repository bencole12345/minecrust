use na::{Point3, Vector3};

/// A point illumination source, whose radiance disperses equally in all directions
#[derive(Debug)]
pub struct PointLight {
    pub position: Point3<f32>,
    pub colour: Vector3<f32>,
    pub intensity: f32,
}

/// A global illumination source, whose radiance is always observed as being received from the same
/// direction and intensity, regardless of the location of the observer
#[derive(Debug)]
pub struct GlobalLight {
    // TODO: See if there's a fancier type to guarantee that it's a unit vector
    pub direction: Vector3<f32>,
    pub colour: Vector3<f32>,
    pub intensity: f32,
}
// Lighting information about a scene that can be rendered
#[derive(Debug)]
pub struct SceneLighting {
    pub point_lights: Vec<PointLight>,
    pub global_light: GlobalLight,
}
