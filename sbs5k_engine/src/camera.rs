use std::f32::consts::PI;

use nalgebra::{Matrix4, Point3, Rotation3, Translation, Vector3};

// TODO: Unify with higher-level types
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
        let rotation_yaw = Rotation3::from_axis_angle(&Vector3::y_axis(), self.yaw + PI);
        let rotation_pitch = Rotation3::new(Vector3::x() * -self.pitch);
        let translation = Translation::from(Vector3::new(0.0, 0.0, 0.0) - self.position.coords);
        (rotation_pitch * rotation_yaw * translation).to_homogeneous()
    }

    /// Get the projection matrix of the camera, in homogeneous coordinates
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        let aspect = 16.0 / 9.0;
        let fovy = PI * 0.5;
        let znear = 0.1;
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

// TODO: Make these tests work

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use glm::{cos, sin};
//     use rstest::*;

//     #[rstest]
//     fn test_default_view_matrix() {
//         let camera_pos = CameraPosition {
//             position: Point3::origin(),
//             yaw: 0.0,
//             pitch: 0.0,
//         };
//         #[rustfmt::skip]
//         let expected_view_matrix = Matrix4::new(1.0, 0.0, 0.0, 0.0,
//                                                 0.0, 1.0, 0.0, 0.0,
//                                                 0.0, 0.0, 1.0, 0.0,
//                                                 0.0, 0.0, 0.0, 1.0);
//         let actual_view_matrix = camera_pos.view_matrix();
//         assert_eq!(expected_view_matrix, actual_view_matrix);
//     }

//     #[rstest]
//     fn test_view_matrix_translates_correctly() {
//         let camera_pos = CameraPosition {
//             position: Point3::new(1.0, 2.0, 3.0),
//             yaw: 0.0,
//             pitch: 0.0,
//         };
//         #[rustfmt::skip]
//         let expected_view_matrix = Matrix4::new(1.0, 0.0, 0.0, -1.0,
//                                                 0.0, 1.0, 0.0, -2.0,
//                                                 0.0, 0.0, 1.0, -3.0,
//                                                 0.0, 0.0, 0.0,  1.0);
//         let actual_view_matrix = camera_pos.view_matrix();
//         assert_eq!(expected_view_matrix, actual_view_matrix);
//     }

//     #[rstest]
//     fn test_view_matrix_applies_yaw_corectly() {
//         let angle = PI / 4.0;
//         let camera_pos = CameraPosition {
//             position: Point3::origin(),
//             yaw: angle,
//             pitch: 0.0,
//         };
//         #[rustfmt::skip]
//         let expected_view_matrix = Matrix4::new(cos(-angle), 0.0, -sin(-angle), 0.0,
//                                                         0.0, 1.0,          0.0, 0.0,
//                                                 sin(-angle), 0.0,  cos(-angle), 0.0,
//                                                         0.0, 0.0,          0.0, 1.0);
//         let actual_view_matrix = camera_pos.view_matrix();
//         assert_eq!(expected_view_matrix, actual_view_matrix);
//     }

//     // TODO: Tests for pitch

//     // TODO: Tests for composition
// }
