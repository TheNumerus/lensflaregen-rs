use std::rc::Rc;

use crate::{
    gl_wrapper::{framebuffer::Framebuffer, geometry::Geometry, texture::Texture2d},
    lfg::ghost::gen_ghost_geo,
    WindowState,
};

use super::{flare::Flare, ghost::Ghost, shader_lib::ShaderLib};

pub struct Effect {
    pub flare: Flare,
    pub ghosts: Vec<Ghost>,
}

impl Effect {
    pub fn new() -> Self {
        let ghost_geo = Rc::new(gen_ghost_geo(6));

        Self {
            flare: Flare::new(),
            ghosts: vec![Ghost::new(ghost_geo.clone()), Ghost::new(ghost_geo)],
        }
    }

    pub fn draw(&self, shader_lib: &ShaderLib, noise: &Texture2d, main_fb: &Framebuffer, side_fb: &Framebuffer, quad: &Geometry, state: &WindowState) {
        main_fb.bind();
        main_fb.clear();
        for ghost in &self.ghosts {
            side_fb.bind();
            side_fb.clear();

            shader_lib.ghost.bind();
            shader_lib.ghost.set_float_uniform("aspect_ratio", [state.size.0 as f32 / state.size.1 as f32]);
            ghost.draw(&shader_lib.ghost);

            main_fb.bind();
            shader_lib.dispersion.bind();
            shader_lib
                .dispersion
                .set_float_uniform("res", [state.size.0 as f32 / 64.0, state.size.1 as f32 / 64.0]);
            side_fb.bind_as_color_texture(0);
            side_fb.bind_as_color_texture(1);
            noise.bind(2);

            quad.draw()
        }

        main_fb.bind();
        shader_lib.flare.bind();
        shader_lib
            .flare
            .set_float_uniform("res", [state.size.0 as f32 / 64.0, state.size.1 as f32 / 64.0]);
        let relative_pos = state.relative_cursor();
        shader_lib.flare.set_float_uniform("flare_position", [relative_pos.0, relative_pos.1]);
        shader_lib.flare.set_float_uniform("aspect_ratio", [state.size.0 as f32 / state.size.1 as f32]);
        self.flare.draw(&shader_lib.flare, &noise);
    }
}
