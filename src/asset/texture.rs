use std::path::Path;

use gl::types::GLuint;
use image::{RgbaImage, EncodableLayout, ImageError};

pub struct Texture {
    id: GLuint
}

impl Texture {
    pub fn new(path: &Path) -> Result<Self, ImageError> {
        let mut id: GLuint = 0;
        let img: RgbaImage = image::open(path)?.into_rgba8();

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        }

        Ok(Self { id })
    }
    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
