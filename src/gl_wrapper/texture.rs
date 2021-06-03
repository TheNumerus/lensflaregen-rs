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
            TextureFormat::Rgba => width * height * 4,
        };
        assert_eq!(data.len(), expected_len as usize);

        let mut tex_id = 0;
        unsafe {
            gl::GenTextures(1, ptr::addr_of_mut!(tex_id));

            gl::BindTexture(gl::TEXTURE_2D, tex_id);

            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, width as i32, height as i32);

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
}

impl From<TextureFormat> for GLenum {
    fn from(tf: TextureFormat) -> Self {
        match tf {
            TextureFormat::Rgb => gl::RGB,
            TextureFormat::Rgba => gl::RGBA,
        }
    }
}

pub trait TexStorage {
    fn gl_type() -> gl::types::GLenum;
}

impl TexStorage for f32 {
    fn gl_type() -> gl::types::GLenum {
        return gl::FLOAT;
    }
}

impl TexStorage for u8 {
    fn gl_type() -> gl::types::GLenum {
        return gl::UNSIGNED_BYTE;
    }
}
