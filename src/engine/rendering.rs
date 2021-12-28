use std::cmp;

use na::Vector3;
use packer::Packer;

use crate::engine::binding::BindGuard;
use crate::engine::camera::Camera;
use crate::engine::lighting::{GlobalLight, PointLight};
use crate::engine::resources;
use crate::engine::scene::{Scene, SceneObject};
use crate::engine::shaders::{Shader, ShaderProgram, ShaderType};
<<<<<<< HEAD
use crate::engine::texture::{Texture, ImageFileFormat};
=======
use crate::engine::texture::{ImageFileFormat, Texture};
>>>>>>> load-at-compile-time
use crate::engine::uniforms::Uniform;

const BACKGROUND_R: f32 = 0.2;
const BACKGROUND_G: f32 = 0.2;
const BACKGROUND_B: f32 = 0.2;

const MAX_POINT_LIGHTS: usize = 4;

pub struct Renderer {
    cubes_shader_program: ShaderProgram,
    cubes_texture: Texture,
}

impl Renderer {
    pub fn new() -> Renderer {
        let cubes_vertex_shader = Shader::new(
            resources::Shaders::get("cubes.vert").unwrap(),
            ShaderType::VertexShader,
            "cubes.vert",
        );
        let cubes_fragment_shader = Shader::new(
            resources::Shaders::get("cubes.frag").unwrap(),
            ShaderType::FragmentShader,
            "cubes.frag",
        );
        let cubes_shader_program = ShaderProgram::new(cubes_vertex_shader, cubes_fragment_shader);

        let cubes_texture = Texture::new(
            resources::Textures::get("cube.png").unwrap(),
            ImageFileFormat::Png,
        );

        Renderer {
            cubes_shader_program,
            cubes_texture,
        }
    }

    /// Sets up the OpenGL environment ready to use this renderer
    pub fn setup(&mut self) {
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CCW);
            gl::CullFace(gl::BACK);

            gl::Enable(gl::MULTISAMPLE);
        }

        // Set up textures
        // TODO: Allocate these more intelligently
        // TODO: Use a BindGuard for this
        self.cubes_texture.bind_to_texture_unit(gl::TEXTURE0);
        write_texture_uniforms(&self.cubes_shader_program, &self.cubes_texture);
    }

    pub fn render_scene(&self, scene: &Scene, camera: &Camera) {
        unsafe {
            gl::ClearColor(BACKGROUND_R, BACKGROUND_G, BACKGROUND_B, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Bind shader program
        let _shader_program_guard = BindGuard::create_bind(&self.cubes_shader_program);

        // Write all the uniforms
        write_camera_uniforms(&self.cubes_shader_program, camera);
        write_point_light_uniforms(&self.cubes_shader_program, &scene.point_lights);
        write_global_illuminant_uniforms(&self.cubes_shader_program, &scene.global_light);

        // Render each object
        for object in &scene.objects {
            self.render_object(&object);
        }
    }

    fn render_object(&self, object: &SceneObject) {
        let _vertex_data_guard = BindGuard::create_bind(&object.model_data);
        write_model_uniforms(&self.cubes_shader_program, object);
        let first_index = 0;
        let num_vertices = object.model_data.num_vertices() as i32;
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, first_index, num_vertices);
        }
    }
}

fn write_camera_uniforms(program: &ShaderProgram, camera: &Camera) {
    program.write_uniform(Uniform::ViewMatrix(&camera.view_matrix()));
    program.write_uniform(Uniform::ProjectionMatrix(&camera.projection_matrix()));
}

fn write_model_uniforms(program: &ShaderProgram, model: &SceneObject) {
    program.write_uniform(Uniform::ModelMatrix(&model.model_matrix()));
}

fn write_global_illuminant_uniforms(program: &ShaderProgram, global_illuminant: &GlobalLight) {
    program.write_uniform(Uniform::GlobalIlluminantDirection(
        &global_illuminant.direction,
    ));
    program.write_uniform(Uniform::GlobalIlluminantColour(&global_illuminant.colour));
    program.write_uniform(Uniform::GlobalIlluminantIntensity(
        global_illuminant.intensity,
    ));
}

fn write_point_light_uniforms(program: &ShaderProgram, point_lights: &Vec<PointLight>) {
    let mut light_positions: Vec<Vector3<f32>> = vec![];
    let mut light_colours: Vec<Vector3<f32>> = vec![];
    let mut light_intensities: Vec<f32> = vec![];

    let count = cmp::min(point_lights.len(), MAX_POINT_LIGHTS);
    for light in point_lights.iter().take(count) {
        light_positions.push(light.position.coords);
        light_colours.push(light.colour);
        light_intensities.push(light.intensity);
    }

    program.write_uniform(Uniform::PointLightsPositions(&light_positions));
    program.write_uniform(Uniform::PointLightsColours(&light_colours));
    program.write_uniform(Uniform::PointLightsIntensities(&light_intensities));
}

fn write_texture_uniforms(program: &ShaderProgram, texture: &Texture) {
    program.write_uniform(Uniform::CubeTexture(texture));
}
