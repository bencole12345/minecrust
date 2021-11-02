use na::{Matrix4, Vector3};

/// Encodes a uniform value that can be passed to a shader program
pub enum Uniform<'a> {
    Float(f32),
    FloatArray(&'a Vec<f32>),
    // TODO: Add FloatArray(&'a Vec<f32>, dimension: u32),
    Vec3(&'a Vector3<f32>),
    Vec3Array(&'a Vec<Vector3<f32>>),
    Mat4(&'a Matrix4<f32>),
}
