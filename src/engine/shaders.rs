use std::convert::TryInto;
use std::ffi;
use std::ptr;
use std::str;

use gl::types::*;

use crate::engine::binding::Bindable;
use crate::engine::uniforms::Uniform;

#[derive(Debug, Eq, PartialEq)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

/// Wraps a single shader, such as a vertex shader or a fragment shader.
pub struct Shader {
    id: GLuint,
    shader_type: ShaderType,
    debug_name: &'static str,
}

/// Wraps a linked shader program, consisting of both a vertex shader and a
/// fragment shader.
pub struct ShaderProgram {
    id: GLuint,
}

impl Shader {
    pub fn new(code: &[u8], shader_type: ShaderType, debug_name: &'static str) -> Self {
        let code = ffi::CString::new(code).unwrap();
        let gl_shader_type = match &shader_type {
            ShaderType::VertexShader => gl::VERTEX_SHADER,
            ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
        };
        let id = unsafe {
            let id = gl::CreateShader(gl_shader_type);
            gl::ShaderSource(id, 1, &code.as_ptr(), ptr::null());
            gl::CompileShader(id);
            id
        };

        if !compiled_successfully(id) {
            dump_shader_compile_error(id, debug_name);
            panic!("Failed to compile shader");
        }

        Shader {
            id,
            shader_type,
            debug_name,
        }
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
    pub fn new(vertex_shader: Shader, fragment_shader: Shader) -> Self {
        if vertex_shader.shader_type != ShaderType::VertexShader {
            panic!("Bad vertex shader type: {:?}", vertex_shader.shader_type);
        }
        if fragment_shader.shader_type != ShaderType::FragmentShader {
            panic!(
                "Bad fragment shader type: {:?}",
                fragment_shader.shader_type
            );
        }

        let id = unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);
            gl::LinkProgram(id);
            id
        };

        if !linked_successfully(id) {
            dump_shader_link_error(id, vertex_shader.debug_name, fragment_shader.debug_name);
            panic!("Failed to link shader program");
        }

        ShaderProgram { id }
    }

    #[inline]
    fn lookup_uniform_location(&self, name: &str) -> GLint {
        unsafe {
            let name_as_cstring = ffi::CString::new(name).unwrap();
            gl::GetUniformLocation(self.id, name_as_cstring.as_ptr())
        }
    }

    #[inline]
    pub(crate) fn write_uniform(&self, uniform: Uniform) {
        let name = uniform.get_name_in_shader();
        let position = self.lookup_uniform_location(name);
        unsafe {
            match uniform {
                Uniform::ModelMatrix(m) | Uniform::ViewMatrix(m) | Uniform::ProjectionMatrix(m) => {
                    gl::UniformMatrix4fv(position, 1, gl::FALSE, m.as_ptr());
                }
                Uniform::PointLightsPositions(va) | Uniform::PointLightsColours(va) => {
                    if !va.is_empty() {
                        gl::Uniform3fv(position, va.len().try_into().unwrap(), va[0].as_ptr());
                    }
                }
                Uniform::PointLightsIntensities(v) => {
                    if !v.is_empty() {
                        gl::Uniform1fv(position, v.len().try_into().unwrap(), v.as_ptr())
                    };
                }
                Uniform::GlobalIlluminantDirection(v) | Uniform::GlobalIlluminantColour(v) => {
                    gl::Uniform3f(position, v.x, v.y, v.z);
                }
                Uniform::GlobalIlluminantIntensity(intensity) => {
                    gl::Uniform1f(position, intensity);
                }
                // TODO: Work out what's going on here - it seems broken
                Uniform::ModelTexture(_texture_binding) => {
                    gl::Uniform1i(position, 0);
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
    #[inline]
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    #[inline]
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

fn dump_shader_compile_error(shader: GLuint, debug_name: &str) {
    let error_log = read_error_log(shader);
    println!(
        "Failed to compile shader program {}, error:\n{}",
        debug_name, error_log
    );
}

fn dump_shader_link_error(
    shader_program_id: GLuint,
    vertex_shader_debug_name: &str,
    fragment_shader_debug_name: &str,
) {
    let error_log = read_error_log(shader_program_id);
    println!(
        "Failed to link vertex shader {} with fragment shader {}, error:\n{}",
        vertex_shader_debug_name, fragment_shader_debug_name, error_log
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
