use std::f32::consts::PI;

use glm::{cos, fmod, sin};
use na::{Matrix4, Point3, Rotation3, Translation, Vector3};

use crate::engine::movement::Moveable;

// TODO: Use nalgebra functions to get axes as constants

/// Encodes the position of the camera in the game world
pub struct Camera {
    /// Camera's world coordinates in (x, y, z) form
    position: Point3<f32>,

    /// Rotation clockwise from (0, 0, -1) around +y direction in radians
    phi: f32,

    /// Rotation upwards above (x,z) plane towards +y direction in radians
    theta: f32,
}

impl Camera {
    pub fn create_at_origin() -> Self {
        Camera {
            position: Point3::origin(),
            phi: 0.0,
            theta: 0.0,
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let rotation_y_axis = Rotation3::new(Vector3::new(0.0, 1.0, 0.0) * self.phi);
        let rotation_x_axis = Rotation3::new(Vector3::new(1.0, 0.0, 0.0) * -self.theta);
        let rotation = rotation_x_axis * rotation_y_axis;
        let translation = Translation::from(Vector3::new(0.0, 0.0, 0.0) - self.position.coords);
        (rotation * translation).to_homogeneous()
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        let aspect = 16.0 / 9.0;
        let fovy = PI * 0.2;
        let znear = 1.0;
        let zfar = 1000.0;
        Matrix4::new_perspective(aspect, fovy, znear, zfar)
    }
}

impl Moveable for Camera {
    fn move_forwards(&mut self, distance: f32) {
        let x = sin(self.phi) * cos(self.theta);
        let y = sin(self.theta);
        let z = -cos(self.phi) * cos(self.theta);
        let direction = Vector3::new(x, y, z);
        self.position = self.position + direction * distance;
    }

    fn move_backwards(&mut self, distance: f32) {
        let x = -sin(self.phi) * cos(self.theta);
        let y = -sin(self.theta);
        let z = cos(self.phi) * cos(self.theta);
        let direction = Vector3::new(x, y, z);
        self.position = self.position + direction * distance;
    }

    fn move_left(&mut self, distance: f32) {
        let x = -cos(self.phi);
        let y: f32 = 0.0;
        let z = -sin(self.phi);
        let direction = Vector3::new(x as f32, y, z as f32);
        self.position = self.position + direction * distance;
    }

    fn move_right(&mut self, distance: f32) {
        let x = cos(self.phi);
        let y = 0.0;
        let z = sin(self.phi);
        let direction = Vector3::new(x, y, z);
        self.position = self.position + direction * distance;
    }

    fn move_up(&mut self, distance: f32) {
        let up = Vector3::new(0.0, 1.0, 0.0);
        self.position = self.position + up * distance;
    }

    fn move_down(&mut self, distance: f32) {
        let down = Vector3::new(0.0, -1.0, 0.0);
        self.position = self.position + down * distance;
    }

    fn rotate_left(&mut self, angle: f32) {
        self.phi = fmod(self.phi - angle, 2.0 * PI);
    }

    fn rotate_right(&mut self, angle: f32) {
        self.phi = fmod(self.phi + angle, 2.0 * PI);
    }

    fn rotate_up(&mut self, angle: f32) {
        self.theta = glm::clamp(self.theta + angle, -PI * 0.95, PI * 0.95);
    }

    fn rotate_down(&mut self, angle: f32) {
        self.theta = glm::clamp(self.theta - angle, -PI * 0.95, PI * 0.95);
    }
}
