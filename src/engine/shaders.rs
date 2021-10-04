use std::ffi;
use std::fs;
use std::path::Path;
use std::ptr;
use std::str;

extern crate gl;
use gl::types::*;

use super::binding::Bindable;

const LOG_BUFFER_SIZE: usize = 1024;

/// Wraps a single shader, such as a vertex shader or a fragment shader.
struct Shader {
    pub id: GLuint,
}

/// Wraps a linked shader program, consisting of both a vertex shader and a
/// fragment shader.
pub struct ShaderProgram {
    pub id: GLuint,
}

// TODO: Have a think about how uniforms are going to work

impl Shader {
    fn from_path(path: &Path, shader_type: GLenum) -> Shader {
        let buffer = fs::read(path).expect("Failed to load shader"); // TODO: Report the bad path
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
    let mut info_log = Vec::with_capacity(LOG_BUFFER_SIZE);
    unsafe {
        info_log.set_len(LOG_BUFFER_SIZE - 1);
        // info_log.set_len(LOG_BUFFER_SIZE);
        gl::GetShaderInfoLog(
            shader,
            LOG_BUFFER_SIZE as GLsizei,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
    }
    let path_str = path.to_str().unwrap();
    let error_str = str::from_utf8(&info_log).unwrap();
    println!(
        "Failed to compile shader program {}: {}",
        path_str, error_str
    );
}

fn dump_shader_link_error(
    shader_program: GLuint,
    vertex_shader_path: &Path,
    fragment_shader_path: &Path,
) {
    let mut info_log = Vec::with_capacity(LOG_BUFFER_SIZE);
    unsafe {
        info_log.set_len(LOG_BUFFER_SIZE - 1);
        gl::GetProgramInfoLog(
            shader_program,
            LOG_BUFFER_SIZE as i32,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
    }
    let vertex_shader_path_str = vertex_shader_path.to_str().unwrap();
    let fragment_shader_path_str = fragment_shader_path.to_str().unwrap();
    let error_str = str::from_utf8(&info_log).unwrap();
    println!(
        "Failed to link shader using vertex shader {} and fragment shader {}, error: {}",
        vertex_shader_path_str, fragment_shader_path_str, error_str
    );
}