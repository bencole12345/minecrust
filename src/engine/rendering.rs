use std::cmp;
use std::ptr;

use gl::types::*;
use na::Vector3;
use packer::Packer;

use crate::engine::binding::BindGuard;
use crate::engine::camera::Camera;
use crate::engine::lighting::{GlobalLight, PointLight};
use crate::engine::resources;
use crate::engine::scene::{Scene, SceneObject};
use crate::engine::shaders::{Shader, ShaderProgram, ShaderType};
use crate::engine::texture::{ImageFileFormat, Texture};
use crate::engine::uniforms::Uniform;

const BACKGROUND_R: f32 = 0.2;
const BACKGROUND_G: f32 = 0.2;
const BACKGROUND_B: f32 = 0.2;

const MAX_POINT_LIGHTS: usize = 4;

/// A target capable of being rendered to
pub trait RenderingContext {
    fn swap_buffers(&mut self);
}

pub(crate) struct Renderer {
    cubes_shader_program: ShaderProgram,
    cubes_texture: Texture,
    skybox_shader_program: ShaderProgram,
}

impl Renderer {
    pub(crate) fn new() -> Renderer {
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

        let skybox_vertex_shader = Shader::new(
            resources::Shaders::get("skybox.vert").unwrap(),
            ShaderType::VertexShader,
            "skybox.vert",
        );
        let skybox_fragment_shader = Shader::new(
            resources::Shaders::get("skybox.frag").unwrap(),
            ShaderType::FragmentShader,
            "skybox.frag",
        );
        let skybox_shader_program =
            ShaderProgram::new(skybox_vertex_shader, skybox_fragment_shader);

        Renderer {
            cubes_shader_program,
            cubes_texture,
            skybox_shader_program,
        }
    }

    /// Set up the OpenGL environment ready to use this renderer
    pub fn setup(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            // Cull faces oriented away from the camera to avoid wasted work
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CCW);
            gl::CullFace(gl::BACK);

            // Enable multisampling for anti-aliasing
            gl::Enable(gl::MULTISAMPLE);
        }

        // Set up textures
        // TODO: Allocate these more intelligently + consider integrating with BindGuard
        self.cubes_texture.bind_to_texture_unit(gl::TEXTURE0);
        write_texture_uniforms(&self.cubes_shader_program, &self.cubes_texture);
    }

    /// Render a scene to a `RenderingContext`
    pub fn render_scene<'a, T>(&self, scene: &Scene, camera: &Camera, target: &'a mut T)
    where
        T: RenderingContext,
    {
        unsafe {
            gl::ClearColor(BACKGROUND_R, BACKGROUND_G, BACKGROUND_B, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.render_objects(scene, camera);
        self.render_skybox(scene, camera);

        target.swap_buffers();
    }

    #[inline]
    fn render_objects(&self, scene: &Scene, camera: &Camera) {
        // Bind shader program
        let _shader_program_guard = BindGuard::create_bind(&self.cubes_shader_program);

        // Write all the uniforms
        write_camera_uniforms(&self.cubes_shader_program, camera);
        write_point_light_uniforms(&self.cubes_shader_program, &scene.point_lights);
        write_global_illuminant_uniforms(&self.cubes_shader_program, &scene.global_light);

        // Render each object
        for object in &scene.objects {
            // Bind this object's vertex data
            let _vertex_data_guard = BindGuard::create_bind(&object.model_data);

            // Write uniforms specific to this object
            write_model_uniforms(&self.cubes_shader_program, object);

            // Do the render
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    object.model_data.num_elements() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null_mut(),
                );
            }
        }
    }

    #[inline]
    fn render_skybox(&self, scene: &Scene, camera: &Camera) {
        // Bind shader program
        let _shader_program_guard = BindGuard::create_bind(&self.skybox_shader_program);

        // Write the uniforms we need
        write_camera_uniforms(&self.skybox_shader_program, camera);

        // Save the old depth function and
        let mut old_depth_func: GLint = 0;
        let mut old_cull_face_mode: GLint = 0;
        unsafe {
            gl::GetIntegerv(gl::DEPTH_FUNC, &mut old_depth_func);
            gl::GetIntegerv(gl::CULL_FACE_MODE, &mut old_cull_face_mode);
        }

        // Set the new modes we need
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
            gl::CullFace(gl::FRONT);
        }

        // Do the render
        let _skybox_cube_guard = BindGuard::create_bind(&scene.skybox.model);
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                scene.skybox.model.num_elements() as i32,
                gl::UNSIGNED_INT,
                ptr::null_mut(),
            );
        }

        // Restore the old settings
        unsafe {
            gl::DepthFunc(old_depth_func as u32);
            gl::CullFace(old_cull_face_mode as u32);
        }
    }
}

#[inline]
fn write_camera_uniforms(program: &ShaderProgram, camera: &Camera) {
    program.write_uniform(Uniform::ViewMatrix(&camera.view_matrix()));
    program.write_uniform(Uniform::ProjectionMatrix(&camera.projection_matrix()));
}

#[inline]
fn write_model_uniforms(program: &ShaderProgram, model: &SceneObject) {
    program.write_uniform(Uniform::ModelMatrix(&model.model_matrix()));
}

#[inline]
fn write_global_illuminant_uniforms(program: &ShaderProgram, global_illuminant: &GlobalLight) {
    program.write_uniform(Uniform::GlobalIlluminantDirection(
        &global_illuminant.direction,
    ));
    program.write_uniform(Uniform::GlobalIlluminantColour(&global_illuminant.colour));
    program.write_uniform(Uniform::GlobalIlluminantIntensity(
        global_illuminant.intensity,
    ));
}

#[inline]
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

#[inline]
fn write_texture_uniforms(program: &ShaderProgram, texture: &Texture) {
    program.write_uniform(Uniform::CubeTexture(texture));
}
