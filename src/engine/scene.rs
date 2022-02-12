use na::{Matrix4, Point3, Translation3, Vector3};

use crate::engine::model::Model;

/// An object present in a `Scene`
#[derive(Debug)]
pub struct SceneObject {
    /// The position of the object in world coordinates
    pub position: Point3<f32>,

    /// The orientation of the object in the world
    pub orientation: Vector3<f32>,

    /// The scale of the object relative to the `ModelData`
    pub scale: f32,

    /// The model data for this object
    pub model: Model,
}

impl SceneObject {
    /// Compute the model matrix for this scene object
    pub fn model_matrix(&self) -> Matrix4<f32> {
        compute_model_matrix(self.position)
    }
}

#[inline]
fn compute_model_matrix(position: Point3<f32>) -> Matrix4<f32> {
    Translation3::from(position).to_homogeneous()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_model_matrix_correct_for_object_at_origin() {
        let object_pos = Point3::origin();
        #[rustfmt::skip]
        let expected_model_matrix = Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                                 0.0, 1.0, 0.0, 0.0,
                                                 0.0, 0.0, 1.0, 0.0,
                                                 0.0, 0.0, 0.0, 1.0);
        let actual = compute_model_matrix(object_pos);
        assert_eq!(expected_model_matrix, actual);
    }

    #[rstest]
    fn test_model_matrix_correct_for_object_not_at_origin() {
        let object_pos = Point3::new(1.0, 2.0, 3.0);
        #[rustfmt::skip]
        let expected_model_matrix = Matrix4::new(1.0, 0.0, 0.0, 1.0,
                                                 0.0, 1.0, 0.0, 2.0,
                                                 0.0, 0.0, 1.0, 3.0,
                                                 0.0, 0.0, 0.0, 1.0);
        let actual = compute_model_matrix(object_pos);
        assert_eq!(expected_model_matrix, actual);
    }
}
