use na::{Matrix4, Vector3};

use crate::engine::texture::TextureBinding;

/// The supported uniforms that can be passed to a shader program
pub(crate) enum Uniform<'a> {
    /// The model matrix for the object being rendered
    ModelMatrix(&'a Matrix4<f32>),

    /// The view matrix for the camera
    ViewMatrix(&'a Matrix4<f32>),

    /// The projection matrix for the camera
    ProjectionMatrix(&'a Matrix4<f32>),

    /// The positions of the scene's point light sources in world-space
    PointLightsPositions(&'a Vec<Vector3<f32>>),

    /// The colours of the scene's point light sources
    PointLightsColours(&'a Vec<Vector3<f32>>),

    /// The radiant intensities of the scene's point light sources
    PointLightsIntensities(&'a Vec<f32>),

    /// The direction of the scene's global illuminant in world-space
    GlobalIlluminantDirection(&'a Vector3<f32>),

    /// The colour of the scene's global illuminant
    GlobalIlluminantColour(&'a Vector3<f32>),

    /// The radiant intensity of the scene's global illuminant
    GlobalIlluminantIntensity(f32),

    /// The texture to be used when rendering a model
    ModelTexture(&'a TextureBinding),
}

impl<'a> Uniform<'a> {
    #[inline]
    pub(crate) const fn get_name_in_shader(&self) -> &str {
        // TODO: Use PHF (https://crates.io/crates/phf) to speed this up
        match self {
            Uniform::ModelMatrix(_) => "Model",
            Uniform::ViewMatrix(_) => "View",
            Uniform::ProjectionMatrix(_) => "Projection",
            Uniform::PointLightsPositions(_) => "pointLights.positions",
            Uniform::PointLightsColours(_) => "pointLights.colours",
            Uniform::PointLightsIntensities(_) => "pointLights.intensities",
            Uniform::GlobalIlluminantDirection(_) => "globalIlluminant.direction",
            Uniform::GlobalIlluminantColour(_) => "globalIlluminant.colour",
            Uniform::GlobalIlluminantIntensity(_) => "globalIlluminant.intensity",
            // TODO: Investigate why the texture still seems to work if I spell the uniform name wrong
            Uniform::ModelTexture(_) => "modelTexture",
        }
    }
}
