use std::cmp;
use std::ptr;

use gl::types::*;
use nalgebra::Vector3;

use crate::binding::BindGuard;
use crate::camera::CameraPosition;
use crate::fog::FogParameters;
use crate::lighting::{GlobalLight, PointLight, SceneLighting};
use crate::resources;
use crate::scene::SceneObject;
use crate::shaders::{Shader, ShaderProgram};
use crate::skybox::Skybox;
use crate::texture::TextureBinding;
use crate::uniforms::Uniform;

const BACKGROUND_R: f32 = 0.2;
const BACKGROUND_G: f32 = 0.2;
const BACKGROUND_B: f32 = 0.2;

const MAX_POINT_LIGHTS: usize = 4;

/// A physical display to which a buffer can be displayed
pub trait DisplayTarget {
    fn swap_buffers(&mut self);
}

/// A logical target to which scene objects can be rendered
pub trait RenderTarget {
    /// Render objects to the target
    fn render_objects(
        &self,
        objects: &[&SceneObject],
        lighting: &SceneLighting,
        camera: &CameraPosition,
        fog: &FogParameters,
    );

    /// Render a skybox to the active render target
    fn render_skybox(&self, skybox: &Skybox, camera: &CameraPosition);
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
        let scene_objs_vert = Shader::new(&resources::shaders::SCENE_OBJECTS_VERT_SHADERS);
        let scene_objects_frag = Shader::new(&resources::shaders::SCENE_OBJECTS_FRAG_SHADER);
        let cubes_shader_program = ShaderProgram::new(scene_objs_vert, scene_objects_frag);

        let skybox_vert = Shader::new(&resources::shaders::SKYBOX_VERT_SHADER);
        let skybox_frag = Shader::new(&resources::shaders::SKYBOX_FRAG_SHADER);
        let skybox_shader_program = ShaderProgram::new(skybox_vert, skybox_frag);

        // Initial setup of the OpenGL environment
        unsafe {
            // Depth test so that the Z-buffer works
            gl::Enable(gl::DEPTH_TEST);

            // Cull faces oriented away from the camera to avoid wasted work
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CCW);
            gl::CullFace(gl::BACK);

            // Enable multisampling for anti-aliasing
            gl::Enable(gl::MULTISAMPLE);
        }

        Renderer {
            cubes_shader_program,
            skybox_shader_program,
        }
    }

    /// Commence a render pass
    #[inline(always)]
    fn begin_render_pass(&self, _target: &impl DisplayTarget) {
        unsafe {
            gl::ClearColor(BACKGROUND_R, BACKGROUND_G, BACKGROUND_B, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    /// Finalise a render pass
    ///
    /// This function will block until the GPU has finished all buffered draw calls.
    #[inline(always)]
    fn complete_render_pass(&self, target: &mut impl DisplayTarget) {
        unsafe {
            // TODO: Decide policy around this
            gl::Finish();
        }
        target.swap_buffers();
    }

    /// Render to a `RenderTarget`.
    ///
    /// The display will be automatically cleared before rendering and the buffers swapped after.
    /// The supplied `render_impl` should contain *all* draw commands for this frame.
    pub fn do_render_pass<F>(&mut self, target: &mut impl DisplayTarget, render_impl: &F)
    where
        F: Fn(&mut dyn RenderTarget),
    {
        self.begin_render_pass(target);
        render_impl(self);
        self.complete_render_pass(target);
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer::new()
    }
}

impl RenderTarget for Renderer {
    fn render_objects(
        &self,
        objects: &[&SceneObject],
        scene: &SceneLighting,
        camera: &CameraPosition,
        fog: &FogParameters,
    ) {
        // Bind shader program
        let _shader_program_guard = BindGuard::create_bind(&self.cubes_shader_program);

        // Write all the uniforms
        write_camera_uniforms(&self.cubes_shader_program, camera);
        write_point_light_uniforms(&self.cubes_shader_program, &scene.point_lights);
        write_global_illuminant_uniforms(&self.cubes_shader_program, &scene.global_light);
        write_fog_uniforms(&self.cubes_shader_program, fog);

        // Render each object
        for object in objects.iter() {
            // Set up textures
            // TODO: Allocate these more intelligently + consider integrating with BindGuard
            // TODO: Don't keep writing the same texture data
            let texture_binding = TextureBinding::new(&object.model.texture, gl::TEXTURE0);
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

    fn render_skybox(&self, skybox: &Skybox, camera: &CameraPosition) {
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
    program.write_uniform(Uniform::CameraPosition(&camera.position.coords));
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
fn write_point_light_uniforms(program: &ShaderProgram, point_lights: &[PointLight]) {
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

#[inline]
fn write_fog_uniforms(program: &ShaderProgram, fog_parameters: &FogParameters) {
    program.write_uniform(Uniform::FogNearDistance(fog_parameters.start_threshold));
    program.write_uniform(Uniform::FogFarDistance(fog_parameters.end_threshold));
}
