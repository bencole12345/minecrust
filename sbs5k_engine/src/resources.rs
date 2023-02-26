use crate::shaders::{ShaderSrc, ShaderType};

macro_rules! shader {
    ($name: expr, $shader_type: expr) => {{
        ShaderSrc {
            src: include_bytes!(concat!("../../shaders/", $name)),
            debug_name: $name,
            shader_type: $shader_type,
        }
    }};
}

pub(crate) mod shaders {
    use super::*;

    pub(crate) static SCENE_OBJECTS_VERT_SHADERS: ShaderSrc =
        shader!("scene_objects.vert", ShaderType::VertexShader);
    pub(crate) static SCENE_OBJECTS_FRAG_SHADER: ShaderSrc =
        shader!("scene_objects.frag", ShaderType::FragmentShader);

    pub(crate) static SKYBOX_VERT_SHADER: ShaderSrc = shader!("skybox.vert", ShaderType::VertexShader);
    pub(crate) static SKYBOX_FRAG_SHADER: ShaderSrc = shader!("skybox.frag", ShaderType::FragmentShader);
}
