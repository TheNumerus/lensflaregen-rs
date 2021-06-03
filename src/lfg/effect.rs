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
            ghosts: vec![Ghost::new()],
        }
    }

    pub fn draw(&self, shader_lib: &ShaderLib, noise: &Texture2d, main_fb: &Framebuffer, side_fb: &Framebuffer) {
        for ghost in &self.ghosts {
            side_fb.bind();
            side_fb.clear();
            ghost.draw(&shader_lib.ghost);
        }

        main_fb.bind();
        main_fb.clear();

        self.flare.draw(&shader_lib.flare, &noise);
    }
}
