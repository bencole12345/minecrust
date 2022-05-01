use std::convert::TryInto;
use std::os;

use gl::types::*;
use image::ImageFormat;

/// Holds a texture that can be passed to a shader program
#[derive(Debug)]
pub struct Texture {
    pub(crate) texture_id: GLuint,
}

/// A coordinate into a texture file
///
/// For any valid coordinate, `u` and `v` must both fall in the range [0, 1].
#[derive(Debug)]
pub struct TextureCoordinate {
    pub u: f32,
    pub v: f32,
}

/// The file format of the image to be loaded
#[derive(Debug)]
pub enum ImageFileFormat {
    Png,
    #[allow(dead_code)]
    Guess,
}

/// Represents the binding of a `Texture` to a particular texture unit on the GPU
///
/// The texture is bound to the texture unit at the time of creation, and will be automatically
/// unbound when this object is dropped.
#[derive(Debug)]
pub(crate) struct TextureBinding {
    pub(crate) texture_unit: GLenum,
}

impl Texture {
    pub fn new(buffer: &[u8], format: ImageFileFormat) -> Self {
        let image_option = match format {
            ImageFileFormat::Png => image::load_from_memory_with_format(buffer, ImageFormat::Png),
            ImageFileFormat::Guess => image::load_from_memory(buffer),
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

        Texture { texture_id }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

impl TextureBinding {
    pub(crate) fn new(texture: &Texture, texture_unit: GLenum) -> Self {
        unsafe {
            gl::ActiveTexture(texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id);
        }
        TextureBinding { texture_unit }
    }
}

impl Drop for TextureBinding {
    fn drop(&mut self) {
        unsafe {
            gl::ActiveTexture(self.texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
