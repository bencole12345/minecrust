use std::f32::consts::PI;

use na::{Matrix4, Point3, Rotation3, Translation, Vector3};

// TODO: Use nalgebra functions to get axes as constants

/// Encodes the position of the camera in the game world
pub struct CameraPosition {
    /// Camera's world coordinates in (x, y, z) form
    pub position: Point3<f32>,

    /// Rotation clockwise from (0, 0, -1) around +y direction in radians
    pub yaw: f32,

    /// Rotation upwards above (x,z) plane towards +y direction in radians
    pub pitch: f32,
}

impl CameraPosition {
    /// Get the view matrix of the camera, in homogeneous coordinates
    pub fn view_matrix(&self) -> Matrix4<f32> {
        let rotation_y_axis = Rotation3::new(Vector3::new(0.0, 1.0, 0.0) * self.yaw);
        let rotation_x_axis = Rotation3::new(Vector3::new(1.0, 0.0, 0.0) * -self.pitch);
        let rotation = rotation_x_axis * rotation_y_axis;
        let translation = Translation::from(Vector3::new(0.0, 0.0, 0.0) - self.position.coords);
        (rotation * translation).to_homogeneous()
    }

    /// Get the projection matrix of the camera, in homogeneous coordinates
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        let aspect = 16.0 / 9.0;
        let fovy = PI * 0.25;
        let znear = 1.0;
        let zfar = 1000.0;
        Matrix4::new_perspective(aspect, fovy, znear, zfar)
    }
}

impl Default for CameraPosition {
    fn default() -> Self {
        CameraPosition {
            position: Point3::new(0.0, 64.0, 18.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}
