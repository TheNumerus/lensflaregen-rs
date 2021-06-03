use crate::gl_wrapper::texture::Texture2d;

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

    pub fn draw(&self, shader_lib: &ShaderLib, noise: &Texture2d) {
        self.flare.draw(&shader_lib.flare, &noise);

        for ghost in &self.ghosts {
            ghost.draw(&shader_lib.ghost);
        }
    }
}
