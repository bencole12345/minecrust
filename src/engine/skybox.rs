use crate::engine::model::{VertexData, VertexDataLayoutInfo};

#[rustfmt::skip]
const SKYBOX_CUBE_VERTICES: [f32; 72] = [
    // Position
    // x,    y,    z

    // FRONT
    -0.5, -0.5,  0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,

    // BACK
     0.5,  0.5, -0.5,
     0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5,  0.5, -0.5,

    // LEFT
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5,

    // RIGHT
     0.5,  0.5,  0.5,
     0.5, -0.5,  0.5,
     0.5, -0.5, -0.5,
     0.5,  0.5, -0.5,

    // TOP
    -0.5,  0.5, -0.5,
    -0.5,  0.5,  0.5,
     0.5,  0.5,  0.5,
     0.5,  0.5, -0.5,

    // BOTTOM
    -0.5, -0.5,  0.5,
    -0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5,  0.5,
];

#[rustfmt::skip]
const SKYBOX_CUBE_INDICES: [u32; 36] = [
    // FRONT
    0, 1, 2,
    2, 3, 0,

    // BACK
    4, 5, 6,
    6, 7, 4,

    // LEFT
    8, 9, 10,
    10, 11, 8,

    // RIGHT
    12, 13, 14,
    14, 15, 12,

    // TOP
    16, 17, 18,
    18, 19, 16,

    // BOTTOM
    20, 21, 22,
    22, 23, 20,
];

/// A skybox that can be rendered in a scene
#[derive(Debug)]
pub struct Skybox {
    pub model: VertexData,
}

impl Skybox {
    pub fn new() -> Self {
        let layout_info = VertexDataLayoutInfo {
            position_offset: 0,
            normal_offset: None,
            texture_offset: None,
        };
        let model = VertexData::new(&SKYBOX_CUBE_VERTICES, &SKYBOX_CUBE_INDICES, layout_info);

        Skybox { model }
    }
}
