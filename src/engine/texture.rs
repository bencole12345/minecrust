use std::convert::TryInto;
use std::os;

use gl::types::*;
use image::ImageFormat;

/// Holds a texture that can be passed to a shader program
pub struct Texture {
    pub texture_id: GLuint,
    current_texture_unit: Option<GLenum>,
}

pub enum ImageFileFormat {
    Png,

    #[allow(dead_code)]
    Guess
}

impl Texture {
    pub fn new(buffer: &[u8], format: ImageFileFormat) -> Self {
        let image_option = match format {
            ImageFileFormat::Png => image::load_from_memory_with_format(buffer, ImageFormat::Png),
            ImageFileFormat::Guess => image::load_from_memory(buffer)
        };
        let img = image_option
            .expect("Failed to load texture from buffer")
            .to_rgba8();

        let data = img.as_ptr();
        let width: i32 = img.width().try_into().unwrap();
        let height: i32 = img.height().try_into().unwrap();

        let texture_id = unsafe {
            let mut id: GLuint = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            // Texture wrapping (should be irrelevant)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // Texture filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data as *const os::raw::c_void,
            );

            id
        };

        Texture {
            texture_id,
            current_texture_unit: None,
        }
    }

    pub fn bind_to_texture_unit(&mut self, texture_unit: GLenum) {
        unsafe {
            gl::ActiveTexture(texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
        self.current_texture_unit = Some(texture_unit);
    }

    pub fn unbind_from_texture_unit(&mut self) {
        match self.current_texture_unit {
            None => {
                panic!("Can't unbind texture when it's not bound to a texture unit");
            }
            Some(texture_unit) => unsafe {
                gl::ActiveTexture(texture_unit);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            },
        }
        self.current_texture_unit = None;
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.current_texture_unit.is_some() {
            self.unbind_from_texture_unit();
        }
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}
