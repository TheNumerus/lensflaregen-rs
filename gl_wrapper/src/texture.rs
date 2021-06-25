use std::ptr;

use gl::types::GLenum;
use log::debug;

pub struct Texture2d {
    tex_id: u32,
}

impl Texture2d {
    pub fn new<S: TexStorage>(width: u32, height: u32, data: &[S], format: TextureFormat) -> Self {
        let expected_len = match format {
            TextureFormat::Rgb => width * height * 3,
            TextureFormat::Rgba | TextureFormat::Srgba => width * height * 4,
            TextureFormat::R8 => width * height,
        };
        assert_eq!(data.len(), expected_len as usize);

        let mut tex_id = 0;
        unsafe {
            gl::GenTextures(1, ptr::addr_of_mut!(tex_id));

            gl::BindTexture(gl::TEXTURE_2D, tex_id);

            let int_format = match format {
                TextureFormat::Rgb | TextureFormat::Rgba => gl::RGBA8,
                TextureFormat::Srgba => gl::SRGB_ALPHA,
                TextureFormat::R8 => gl::R8,
            };

            gl::TexStorage2D(gl::TEXTURE_2D, 1, int_format, width as i32, height as i32);

            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                width as i32,
                height as i32,
                format.into(),
                S::gl_type(),
                data.as_ptr() as *const _,
            );

            debug!("Texture {} generated", tex_id);
            Self { tex_id }
        }
    }

    pub fn bind(&self, unit: u8) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit as u32);
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
        }
    }
}

impl Drop for Texture2d {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, ptr::addr_of!(self.tex_id)) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextureFormat {
    Rgb,
    Rgba,
    Srgba,
    R8,
}

impl From<TextureFormat> for GLenum {
    fn from(tf: TextureFormat) -> Self {
        match tf {
            TextureFormat::Rgb => gl::RGB,
            TextureFormat::Rgba | TextureFormat::Srgba => gl::RGBA,
            TextureFormat::R8 => gl::RED,
        }
    }
}

pub trait TexStorage {
    fn gl_type() -> gl::types::GLenum;
}

impl TexStorage for f32 {
    fn gl_type() -> gl::types::GLenum {
        gl::FLOAT
    }
}

impl TexStorage for u8 {
    fn gl_type() -> gl::types::GLenum {
        gl::UNSIGNED_BYTE
    }
}
