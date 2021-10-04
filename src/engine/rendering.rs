use std::env;

use super::binding::BindGuard;
use super::camera::Camera;
use super::scene::{Scene, SceneObject};
use super::shaders::ShaderProgram;

const BACKGROUND_R: f32 = 0.2;
const BACKGROUND_G: f32 = 0.2;
const BACKGROUND_B: f32 = 0.2;

pub struct Renderer {
    triangle_shader_program: ShaderProgram,
}

impl Renderer {
    pub fn create() -> Renderer {
        let current_dir = env::current_dir().unwrap();
        let shaders_dir = current_dir.join("shaders");
        let vertex_shader = shaders_dir.join("triangle.vert");
        let fragment_shader = shaders_dir.join("triangle.frag");
        let program = ShaderProgram::from_vertex_fragment_paths(&vertex_shader, &fragment_shader);

        // TODO: Set up depth test and face culling

        Renderer {
            triangle_shader_program: program,
        }
    }

    pub fn render_scene(&self, scene: &Scene, _camera: &Camera) {
        unsafe {
            gl::ClearColor(BACKGROUND_R, BACKGROUND_G, BACKGROUND_B, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // TODO: Set global illuminant uniform
            // TODO: Set point light uniforms
            // TODO: Set camera uniforms

            for object in &scene.objects {
                self.render_object(&object);
            }
        }
    }

    pub fn render_object(&self, object: &SceneObject) {
        let first_index = 0;
        let num_vertices = object.model_data.num_vertices() as i32;

        // Bind shader program and vertex data
        let _shader_program_guard = BindGuard::create_bind(&self.triangle_shader_program);
        let _vertex_data_guard = BindGuard::create_bind(&object.model_data);

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, first_index, num_vertices);
        }
    }
}
