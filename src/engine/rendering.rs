use std::cmp;
use std::ptr;

use gl::types::*;
use na::Vector3;

use crate::engine::binding::BindGuard;
use crate::engine::camera::CameraPosition;
use crate::engine::lighting::{GlobalLight, PointLight, SceneLighting};
use crate::engine::resources;
use crate::engine::scene::SceneObject;
use crate::engine::shaders::{Shader, ShaderProgram, ShaderType};
use crate::engine::skybox::Skybox;
use crate::engine::texture::TextureBinding;
use crate::engine::uniforms::Uniform;

const BACKGROUND_R: f32 = 0.2;
const BACKGROUND_G: f32 = 0.2;
const BACKGROUND_B: f32 = 0.2;

const MAX_POINT_LIGHTS: usize = 4;

/// A target capable of being rendered to
pub trait RenderingContext {
    fn swap_buffers(&mut self);
}

/// An object capable of rendering `SceneObject`s to a `RenderingContext`
///
/// The rendering process centres around *rendering passes*. To render a scene:
///
/// 1. Create a `Renderer` object.
/// 2. Call `Renderer::setup` to activate the renderer.
/// 3. Call `Renderer::begin_rendering_pass` to commence a rendering pass.
/// 4. Issue calls to `Renderer::draw_objects` and `Renderer::draw_skybox` as appropriate.
/// 5. Finalise the rendering pass by calling `Renderer::complete_render_pass`. This call will block
///    until the GPU has finished the render.
///
/// Rendering commands are buffered and executed asynchronously, so `draw_objects()` and
/// `draw_skybox()` may return before the actual render command has been completed. The only point
/// of synchronisation is the `complete_render_pass()` method.
pub struct Renderer {
    cubes_shader_program: ShaderProgram,
    skybox_shader_program: ShaderProgram,
}

impl Renderer {
    pub fn new() -> Renderer {
        let (scene_objects_vertex_shader_bytes, debug_name) =
            resources::scene_objects_vertex_shader();
        let scene_objects_vertex_shader = Shader::new(
            scene_objects_vertex_shader_bytes,
            ShaderType::VertexShader,
            debug_name,
        );
        let (scene_objects_fragment_shader_bytes, debug_name) =
            resources::scene_objects_fragment_shader();
        let scene_objects_fragment_shader = Shader::new(
            scene_objects_fragment_shader_bytes,
            ShaderType::FragmentShader,
            debug_name,
        );
        let cubes_shader_program =
            ShaderProgram::new(scene_objects_vertex_shader, scene_objects_fragment_shader);

        let (skybox_vertex_shader_bytes, debug_name) = resources::skybox_vertex_shader();
        let skybox_vertex_shader = Shader::new(
            skybox_vertex_shader_bytes,
            ShaderType::VertexShader,
            debug_name,
        );
        let (skybox_fragment_shader_bytes, debug_name) = resources::skybox_fragment_shader();
        let skybox_fragment_shader = Shader::new(
            skybox_fragment_shader_bytes,
            ShaderType::FragmentShader,
            debug_name,
        );
        let skybox_shader_program =
            ShaderProgram::new(skybox_vertex_shader, skybox_fragment_shader);

        Renderer {
            cubes_shader_program,
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
    }

    /// Commence a render pass
    pub fn begin_render_pass(&self, _target: &impl RenderingContext) {
        unsafe {
            gl::ClearColor(BACKGROUND_R, BACKGROUND_G, BACKGROUND_B, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    /// Finalise a render pass
    ///
    /// This function will block until the GPU has finished all buffered draw calls.
    pub fn complete_render_pass(&self, target: &mut impl RenderingContext) {
        unsafe {
            // TODO: Decide policy around this
            gl::Finish();
        }
        target.swap_buffers();
    }

    /// Render a series of objects to the active render target
    pub fn render_objects(
        &self,
        objects: &Vec<SceneObject>,
        scene: &SceneLighting,
        camera: &CameraPosition,
    ) {
        // Bind shader program
        let _shader_program_guard = BindGuard::create_bind(&self.cubes_shader_program);

        // Write all the uniforms
        write_camera_uniforms(&self.cubes_shader_program, camera);
        write_point_light_uniforms(&self.cubes_shader_program, &scene.point_lights);
        write_global_illuminant_uniforms(&self.cubes_shader_program, &scene.global_light);

        // Render each object
        for object in objects.iter() {
            // Set up textures
            // TODO: Allocate these more intelligently + consider integrating with BindGuard
            // TODO: Don't keep writing the same texture data
            let texture_binding = object.model.texture.create_binding(gl::TEXTURE0);
            write_texture_uniforms(&self.cubes_shader_program, &texture_binding);

            // Bind this object's vertex data
            let _vertex_data_guard = BindGuard::create_bind(&object.model.vertices);

            // Write uniforms specific to this object
            write_model_uniforms(&self.cubes_shader_program, object);

            // Do the render
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    object.model.vertices.num_elements() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null_mut(),
                );
            }
        }
    }

    /// Render a skybox to the active render target
    pub fn render_skybox(&self, skybox: &Skybox, camera: &CameraPosition) {
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
        let _skybox_cube_guard = BindGuard::create_bind(&skybox.model);
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                skybox.model.num_elements() as i32,
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
fn write_camera_uniforms(program: &ShaderProgram, camera: &CameraPosition) {
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
fn write_texture_uniforms(program: &ShaderProgram, texture_binding: &TextureBinding) {
    program.write_uniform(Uniform::ModelTexture(texture_binding));
}
