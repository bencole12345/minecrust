use std::mem;
use std::os;

use gl::types::*;

use crate::engine::binding::Bindable;

/// Encodes information about the offsets of different data within a buffer of
/// vertex data
#[derive(Debug)]
pub struct VertexDataStructureInfo {
    pub position_offset: u32,
    pub normal_offset: u32,
    pub texture_offset: Option<u32>,
}

impl VertexDataStructureInfo {
    pub fn stride_floats(&self) -> u32 {
        let position_size = 3;
        let normals_size = 3;
        let texture_coords_size = match self.texture_offset {
            Some(_) => 2,
            None => 0,
        };

        position_size + normals_size + texture_coords_size
    }

    pub fn stride_bytes(&self) -> u32 {
        self.stride_floats() * mem::size_of::<GLfloat>() as u32
    }
}

/// Wraps vertex data about a model
#[derive(Debug)]
pub struct ModelData {
    vertices_count: u32,
    vao: u32,
    vbo: u32,
    // TODO: Add support for index buffers
}

impl ModelData {
    pub fn new(vertex_data: Vec<f32>, vertex_data_info: &VertexDataStructureInfo) -> ModelData {
        let float_size = mem::size_of::<GLfloat>();
        let total_buffer_size_bytes = vertex_data.len() * float_size;
        let stride_floats = vertex_data_info.stride_floats();
        let vertices_count = vertex_data.len() as u32 / stride_floats;

        let (vao, vbo) = unsafe {
            // Create a vertex array object
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Now create a vertex buffer object
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Copy data to GPU
            gl::BufferData(
                gl::ARRAY_BUFFER,
                total_buffer_size_bytes as GLsizeiptr,
                vertex_data.as_ptr() as *const f32 as *const os::raw::c_void,
                gl::STATIC_DRAW,
            );

            // Set up vertex position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                vertex_data_info.stride_bytes() as i32,
                (vertex_data_info.position_offset * float_size as u32) as *const os::raw::c_void,
            );
            gl::EnableVertexAttribArray(0);

            // // Set up vertex normals attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                vertex_data_info.stride_bytes() as i32,
                (vertex_data_info.normal_offset * float_size as u32) as *const os::raw::c_void,
            );
            gl::EnableVertexAttribArray(1);

            // // Set up texture coordinates attribute
            if let Some(offset) = vertex_data_info.texture_offset {
                gl::VertexAttribPointer(
                    2,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    vertex_data_info.stride_bytes() as i32,
                    (offset * float_size as u32) as *const os::raw::c_void,
                );
                gl::EnableVertexAttribArray(2);
            }

            // Unbind the buffer and vertex array object now that we're done
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            (vao, vbo)
        };

        ModelData {
            vertices_count,
            vao,
            vbo,
        }
    }

    pub fn num_vertices(&self) -> u32 {
        self.vertices_count
    }
}

impl Drop for ModelData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl Bindable<'_> for ModelData {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            // TODO: Decide whether this call is needed (deferred until we have multiple VAOs!)
            // gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
