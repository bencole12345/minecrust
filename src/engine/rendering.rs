use std::env;

use na::Vector3;

use crate::engine::binding::BindGuard;
use crate::engine::camera::Camera;
use crate::engine::lighting::{GlobalLight, PointLight};
use crate::engine::scene::{Scene, SceneObject};
use crate::engine::shaders::ShaderProgram;
use crate::engine::uniforms::Uniform;

const BACKGROUND_R: f32 = 0.2;
const BACKGROUND_G: f32 = 0.2;
const BACKGROUND_B: f32 = 0.2;

const MODEL_UNIFORM: &str = "Model";
const VIEW_UNIFORM: &str = "View";
const PROJECTION_UNIFORM: &str = "Projection";

const POINT_LIGHTS_POSITIONS_UNIFORM: &str = "pointLights.positions";
const POINT_LIGHTS_COLOURS_UNIFORM: &str = "pointLights.colours";
const POINT_LIGHTS_INTENSITIES_UNIFORM: &str = "pointLights.intensities";

const GLOBAL_ILLUMINANT_DIRECTION_UNIFORM: &str = "globalIlluminant.direction";
const GLOBAL_ILLUMINANT_COLOUR_UNIFORM: &str = "globalIlluminant.colour";
const GLOBAL_ILLUMINANT_INTENSITY_UNIFORM: &str = "globalIlluminant.intensity";

const MAX_POINT_LIGHTS: u32 = 4;

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
        // Bind vertex data
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
    program.write_uniform(VIEW_UNIFORM, Uniform::Mat4(&camera.view_matrix()));
    program.write_uniform(
        PROJECTION_UNIFORM,
        Uniform::Mat4(&camera.projection_matrix()),
    );
}

fn write_model_uniforms(program: &ShaderProgram, model: &SceneObject) {
    program.write_uniform(MODEL_UNIFORM, Uniform::Mat4(&model.model_matrix()));
}

fn write_global_illuminant_uniforms(program: &ShaderProgram, global_illuminant: &GlobalLight) {
    program.write_uniform(
        GLOBAL_ILLUMINANT_DIRECTION_UNIFORM,
        Uniform::Vec3(&global_illuminant.direction),
    );
    program.write_uniform(
        GLOBAL_ILLUMINANT_COLOUR_UNIFORM,
        Uniform::Vec3(&global_illuminant.colour),
    );
    program.write_uniform(
        GLOBAL_ILLUMINANT_INTENSITY_UNIFORM,
        Uniform::Float(global_illuminant.intensity),
    );
}

fn write_point_light_uniforms(program: &ShaderProgram, point_lights: &Vec<PointLight>) {
    if !point_lights.is_empty() {
        let mut light_positions: Vec<Vector3<f32>> = vec![];
        let mut light_colours: Vec<Vector3<f32>> = vec![];
        let mut light_intensities: Vec<f32> = vec![];

        let mut count = 0;
        'lights: for light in point_lights {
            light_positions.push(light.position.coords);
            light_colours.push(light.colour);
            light_intensities.push(light.intensity);

            count += 1;
            if count >= MAX_POINT_LIGHTS {
                break 'lights;
            }
        }

        program.write_uniform(
            POINT_LIGHTS_POSITIONS_UNIFORM,
            Uniform::Vec3Array(&light_positions),
        );
        program.write_uniform(
            POINT_LIGHTS_COLOURS_UNIFORM,
            Uniform::Vec3Array(&light_colours),
        );
        program.write_uniform(
            POINT_LIGHTS_INTENSITIES_UNIFORM,
            Uniform::FloatArray(&light_intensities),
        );
    }
}
