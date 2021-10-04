extern crate glm;
use glm::vec3;

use super::model;
use super::scene;

pub fn make_triangle() -> scene::SceneObject {
    let info = model::VertexDataStructureInfo {
        position_offset: 0,
        normal_offset: 3,
        texture_offset: None,
    };
    #[rustfmt::skip]
    let triangle_vertices: Vec<f32> = vec![
        // x,    y,    z,   nx,   ny,   nz
        -0.5, -0.5,  0.0,  0.0,  0.0, -1.0,
         0.5, -0.5,  0.0,  0.0,  0.0, -1.0,
         0.0,  0.5,  0.0,  0.0,  0.0, -1.0
    ];
    let data = model::ModelData::new(triangle_vertices, &info);
    scene::SceneObject {
        position: vec3(0.0, 0.0, 0.0),
        orientation: vec3(0.0, 0.0, 0.0),
        scale: 1.0,
        model_data: data,
    }
}
