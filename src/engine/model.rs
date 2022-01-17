use std::mem;
use std::os;
use std::rc::Rc;

use gl::types::*;

use crate::engine::binding::Bindable;
use crate::engine::texture::Texture;

/// Encodes information about the offsets of different data within a buffer of
/// vertex data
#[derive(Debug)]
pub struct VertexDataLayoutInfo {
    pub position_offset: u32,
    pub normal_offset: Option<u32>,
    pub texture_offset: Option<u32>,
}

/// Vertex data describing the geometry of a model
#[derive(Debug)]
pub struct VertexData {
    /// The number of vertices in the model
    vertices_count: u32,

    /// The vertex array object ID
    vao: u32,

    /// The vertex buffer object ID
    vbo: u32,

    /// The element buffer object ID
    ebo: u32,
}

/// A renderable model
#[derive(Debug)]
pub struct Model {
    pub vertices: VertexData,
    pub texture: Rc<Texture>,
}

impl VertexDataLayoutInfo {
    pub fn stride_floats(&self) -> u32 {
        let position_size = 3;
        let normals_size = match self.normal_offset {
            Some(_) => 3,
            None => 0,
        };
        let texture_coords_size = match self.texture_offset {
            Some(_) => 2,
            None => 0,
        };

        position_size + normals_size + texture_coords_size
    }

    pub fn stride_bytes(&self) -> u32 {
        self.stride_floats() * mem::size_of::<GLfloat>() as u32
    }

    /// Reports the index of the positions attribute
    pub fn positions_index(&self) -> usize {
        0
    }

    /// Reports the index of the normals attribute
    pub fn normals_index(&self) -> usize {
        if self.normal_offset.is_none() {
            panic!("normals_index() called when there is no normal attribute")
        }

        self.positions_index() + 1
    }

    /// Reports the index of the textures attribute
    pub fn textures_index(&self) -> usize {
        if self.texture_offset.is_none() {
            panic!("textures_index() called when there is no texture attribute")
        }

        if self.normal_offset.is_some() {
            self.normals_index() + 1
        } else {
            self.normals_index()
        }
    }
}

impl VertexData {
    pub fn new(
        vertex_data: &[f32],
        index_buffer: &[u32],
        layout_info: VertexDataLayoutInfo,
    ) -> VertexData {
        let vertex_buffer_size_bytes = vertex_data.len() * mem::size_of::<GLfloat>();
        let index_buffer_size_bytes = index_buffer.len() * mem::size_of::<GLuint>();

        let vertices_count = index_buffer.len() as u32;

        let (vao, vbo, ebo) = unsafe {
            // Create a vertex array object
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Now create a vertex buffer object
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Copy data into it
            gl::BufferData(
                gl::ARRAY_BUFFER,
                vertex_buffer_size_bytes as GLsizeiptr,
                vertex_data.as_ptr() as *const f32 as *const os::raw::c_void,
                gl::STATIC_DRAW,
            );

            // Create an array buffer object
            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            // Copy data into it
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                index_buffer_size_bytes as GLsizeiptr,
                index_buffer.as_ptr() as *const f32 as *const os::raw::c_void,
                gl::STATIC_DRAW,
            );

            // Set up vertex position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                layout_info.stride_bytes() as i32,
                (layout_info.position_offset * mem::size_of::<GLfloat>() as u32)
                    as *const os::raw::c_void,
            );
            gl::EnableVertexAttribArray(layout_info.positions_index() as u32);

            // Set up vertex normals attribute
            if let Some(offset) = layout_info.normal_offset {
                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    layout_info.stride_bytes() as i32,
                    (offset * mem::size_of::<GLfloat>() as u32) as *const os::raw::c_void,
                );
                gl::EnableVertexAttribArray(layout_info.normals_index() as u32);
            }

            // Set up texture coordinates attribute
            if let Some(offset) = layout_info.texture_offset {
                gl::VertexAttribPointer(
                    2,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    layout_info.stride_bytes() as i32,
                    (offset * mem::size_of::<GLfloat>() as u32) as *const os::raw::c_void,
                );
                gl::EnableVertexAttribArray(layout_info.textures_index() as u32);
            }

            // Unbind all the buffers now that we're done
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            (vao, vbo, ebo)
        };

        VertexData {
            vertices_count,
            vao,
            vbo,
            ebo,
        }
    }

    pub fn num_elements(&self) -> u32 {
        self.vertices_count
    }
}

impl Drop for VertexData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl Bindable<'_> for VertexData {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            // TODO: Decide whether this call is needed (deferred until we have multiple VAOs!)
            // gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}
