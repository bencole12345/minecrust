use std::env;

use super::binding::BindGuard;
use super::camera::Camera;
use super::scene::{Scene, SceneObject};
use super::shaders::ShaderProgram;
use crate::engine::shaders::Uniform;

const BACKGROUND_R: f32 = 0.2;
const BACKGROUND_G: f32 = 0.2;
const BACKGROUND_B: f32 = 0.2;

const MODEL_UNIFORM: &str = "Model";
const VIEW_UNIFORM: &str = "View";
const PROJECTION_UNIFORM: &str = "Projection";

pub struct Renderer {
    cubes_shader_program: ShaderProgram,
}

impl Renderer {
    pub fn new() -> Renderer {
        let current_dir = env::current_dir().unwrap();
        let shaders_dir = current_dir.join("shaders");
        let vertex_shader = shaders_dir.join("cubes.vert");
        let fragment_shader = shaders_dir.join("cubes.frag");
        let program = ShaderProgram::from_vertex_fragment_paths(&vertex_shader, &fragment_shader);

        Renderer {
            cubes_shader_program: program,
        }
    }

    /// Sets up the OpenGL environment ready to use this renderer
    pub fn setup(&self) {
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CCW);
            gl::CullFace(gl::BACK);
        }
    }

    pub fn render_scene(&self, scene: &Scene, camera: &Camera) {
        unsafe {
            gl::ClearColor(BACKGROUND_R, BACKGROUND_G, BACKGROUND_B, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // TODO: Set global illuminant uniform
            // TODO: Set point light uniforms
            // TODO: Set camera uniforms

            for object in &scene.objects {
                self.render_object(&object, camera);
            }
        }
    }

    fn render_object(&self, object: &SceneObject, camera: &Camera) {
        let first_index = 0;
        let num_vertices = object.model_data.num_vertices() as i32;

        // Bind shader program and vertex data
        // let _shader_program_guard = BindGuard::create_bind(&self.triangle_shader_program);

        let _shader_program_guard = BindGuard::create_bind(&self.cubes_shader_program);
        let _vertex_data_guard = BindGuard::create_bind(&object.model_data);

        self.cubes_shader_program
            .write_uniform(MODEL_UNIFORM, Uniform::Mat4(object.model_matrix()));
        self.cubes_shader_program
            .write_uniform(VIEW_UNIFORM, Uniform::Mat4(camera.view_matrix()));
        self.cubes_shader_program.write_uniform(
            PROJECTION_UNIFORM,
            Uniform::Mat4(camera.projection_matrix()),
        );

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, first_index, num_vertices);
        }
    }
}
