use crate::{
    gl_wrapper::{framebuffer::Framebuffer, geometry::Geometry},
    WindowState,
};

use super::{flare::Flare, ghost::Ghost, shader_lib::ShaderLib};

pub struct Effect {
    pub flare: Flare,
    pub ghosts: Vec<Ghost>,
    pub rotation: f32,
    pub blades: u8,
}

impl Effect {
    pub fn new() -> Self {
        Self {
            flare: Flare::new(),
            ghosts: vec![
                Ghost { ..Default::default() },
                Ghost {
                    offset: 0.2,
                    size: 5.0,
                    color: [1.0, 0.5, 0.5, 1.0],
                    ..Default::default()
                },
                Ghost {
                    offset: -0.8,
                    size: 20.0,
                    ..Default::default()
                },
                Ghost {
                    offset: -0.4,
                    size: 15.0,
                    ..Default::default()
                },
            ],
            rotation: 0.2,
            blades: 8,
        }
    }

    pub fn draw(
        &self,
        shader_lib: &ShaderLib,
        main_fb: &mut Framebuffer,
        side_fb: &mut Framebuffer,
        quad: &Geometry,
        ghost_geo: &Geometry,
        state: &WindowState,
    ) {
        // clear main frame
        main_fb.draw_with(|fb| fb.clear());

        for ghost in &self.ghosts {
            // render ghost geometry
            side_fb.draw_with(|fb| {
                fb.clear();

                shader_lib.ghost.bind();
                shader_lib.ghost.set_float_uniform("aspect_ratio", [state.size.0 as f32 / state.size.1 as f32]);
                ghost.draw(&shader_lib.ghost, state.relative_cursor(), ghost_geo, self);
            });

            // copy distorted ghost geometry
            main_fb.draw_with(|_fb| {
                shader_lib.dispersion.bind();
                shader_lib
                    .dispersion
                    .set_float_uniform("res", [state.size.0 as f32 / 64.0, state.size.1 as f32 / 64.0]);
                side_fb.bind_as_color_texture(0);

                ghost.draw_dispersed(&shader_lib.dispersion, &quad);
            });
        }

        // render flare on top
        main_fb.draw_with(|_fb| {
            shader_lib.flare.bind();
            shader_lib
                .flare
                .set_float_uniform("res", [state.size.0 as f32 / 64.0, state.size.1 as f32 / 64.0]);
            let relative_pos = state.relative_cursor();
            shader_lib.flare.set_float_uniform("flare_position", [relative_pos.0, relative_pos.1]);
            shader_lib.flare.set_float_uniform("aspect_ratio", [state.size.0 as f32 / state.size.1 as f32]);
            shader_lib.flare.set_float_uniform("blades", [self.blades as f32]);
            self.flare.draw(&shader_lib.flare, &quad);
        });
    }
}
