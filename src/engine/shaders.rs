use std::convert::TryInto;
use std::ffi;
use std::fs;
use std::path::Path;
use std::ptr;
use std::str;

use gl::types::*;

use crate::engine::binding::Bindable;
use crate::engine::uniforms::Uniform;

/// Wraps a single shader, such as a vertex shader or a fragment shader.
struct Shader {
    id: GLuint,
}

/// Wraps a linked shader program, consisting of both a vertex shader and a
/// fragment shader.
pub struct ShaderProgram {
    id: GLuint,
}

impl Shader {
    fn from_path(path: &Path, shader_type: GLenum) -> Shader {
        let buffer = fs::read(path).expect("Failed to load shader"); // TODO: Report the bad path
                                                                     // let buffer_length = buffer.len();
        let buffer_c_str = ffi::CString::new(buffer).unwrap();

        let shader_id = unsafe {
            let id = gl::CreateShader(shader_type);
            gl::ShaderSource(id, 1, &buffer_c_str.as_ptr(), ptr::null());
            gl::CompileShader(id);
            id
        };

        if !compiled_successfully(shader_id) {
            dump_shader_compile_error(shader_id, path);
            panic!("Failed to compile shader");
        }

        Shader { id: shader_id }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn from_vertex_fragment_paths(
        vertex_shader_path: &Path,
        fragment_shader_path: &Path,
    ) -> ShaderProgram {
        let vertex_shader = Shader::from_path(vertex_shader_path, gl::VERTEX_SHADER);
        let fragment_shader = Shader::from_path(fragment_shader_path, gl::FRAGMENT_SHADER);

        let shader_program = unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);
            gl::LinkProgram(id);
            id
        };

        if !linked_successfully(shader_program) {
            dump_shader_link_error(shader_program, vertex_shader_path, fragment_shader_path);
            panic!("Failed to link shader program");
        }

        ShaderProgram { id: shader_program }
    }

    fn lookup_uniform_location(&self, name: &str) -> GLint {
        unsafe {
            let name_as_cstring = ffi::CString::new(name).unwrap();
            gl::GetUniformLocation(self.id, name_as_cstring.as_ptr())
        }
    }

    pub fn write_uniform(&self, uniform: Uniform) {
        let name = uniform.get_name_in_shader();
        let position = self.lookup_uniform_location(name);
        unsafe {
            match uniform {
                Uniform::ModelMatrix(m) | Uniform::ViewMatrix(m) | Uniform::ProjectionMatrix(m) => {
                    gl::UniformMatrix4fv(position, 1, gl::FALSE, m.as_ptr());
                }
                Uniform::PointLightsPositions(va) | Uniform::PointLightsColours(va) => {
                    gl::Uniform3fv(position, va.len().try_into().unwrap(), va[0].as_ptr());
                }
                Uniform::PointLightsIntensities(v) => {
                    gl::Uniform1fv(position, v.len().try_into().unwrap(), v.as_ptr());
                }
                Uniform::GlobalIlluminantDirection(v) | Uniform::GlobalIlluminantColour(v) => {
                    gl::Uniform3f(position, v.x, v.y, v.z);
                }
                Uniform::GlobalIlluminantIntensity(intensity) => {
                    gl::Uniform1f(position, intensity);
                }
                Uniform::CubeTexture(texture) => {
                    gl::Uniform1i(position, texture.texture_id as GLint);
                }
            }
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Bindable<'_> for ShaderProgram {
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn unbind(&self) {
        // TODO: Consider resetting uniform values here
        unsafe {
            gl::UseProgram(0);
        }
    }
}

fn compiled_successfully(shader_id: GLuint) -> bool {
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
    }
    success == gl::TRUE as GLint
}

fn linked_successfully(shader_program_id: GLuint) -> bool {
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut success);
    }
    success == gl::TRUE as GLint
}

fn dump_shader_compile_error(shader: GLuint, path: &Path) {
    let error_log = read_error_log(shader);
    let path_str = path.to_str().unwrap();

    println!(
        "Failed to compile shader program {}, error:\n{}",
        path_str, error_log
    );
}

fn dump_shader_link_error(
    shader_program: GLuint,
    vertex_shader_path: &Path,
    fragment_shader_path: &Path,
) {
    let error_log = read_error_log(shader_program);
    let vertex_shader_path_str = vertex_shader_path.to_str().unwrap();
    let fragment_shader_path_str = fragment_shader_path.to_str().unwrap();

    println!(
        "Failed to link {} and {}, error:\n{}",
        vertex_shader_path_str, fragment_shader_path_str, error_log
    );
}

fn read_error_log(shader_program: GLuint) -> String {
    // Read error log length
    let mut len: GLint = 0;
    unsafe {
        gl::GetShaderiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
    }

    // Create a buffer of that length and fill it with null characters
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b'\0'].iter().cycle().take(len as usize));

    // Read the info log from OpenGL
    unsafe {
        gl::GetShaderInfoLog(
            shader_program,
            len,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut GLchar,
        );
    }

    String::from_utf8(buffer).expect("Invalid UTF-8 encoding in OpenGL info log")
}
