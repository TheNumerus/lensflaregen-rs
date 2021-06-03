use crate::gl_wrapper::{framebuffer::Framebuffer, texture::Texture2d};

use super::{flare::Flare, ghost::Ghost, shader_lib::ShaderLib};

pub struct Effect {
    pub flare: Flare,
    pub ghosts: Vec<Ghost>,
}

impl Effect {
    pub fn new() -> Self {
        Self {
            flare: Flare::new(),
            ghosts: Vec::new(),
        }
    }

    pub fn draw(&self, shader_lib: &ShaderLib, noise: &Texture2d, main_fb: &Framebuffer, side_fb: &Framebuffer) {
        for ghost in &self.ghosts {
            side_fb.bind();
            ghost.draw(&shader_lib.ghost);
        }

        main_fb.bind();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.flare.draw(&shader_lib.flare, &noise);
    }
}
