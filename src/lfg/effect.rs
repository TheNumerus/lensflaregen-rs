use crate::window_state::WindowState;
use cgmath::{Matrix4, Rad};
use gl_wrapper::{framebuffer::Framebuffer, geometry::Geometry};

use super::{flare::Flare, ghost::Ghost, shader_lib::ShaderLib};

pub struct Effect {
    pub flare: Flare,
    pub ghosts: Vec<Ghost>,
    pub rotation: f32,
    pub blades: u8,
    pub pos_x: f32,
    pub pos_y: f32,
    pub samples: u16,
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
            pos_x: 0.5,
            pos_y: 0.5,
            samples: 8,
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

        let ghost_rotation = Matrix4::from_angle_z(Rad(self.rotation));

        for ghost in &self.ghosts {
            // render ghost geometry
            side_fb.draw_with(|fb| {
                fb.clear();

                shader_lib.ghost.bind();
                shader_lib.ghost.set_float_uniform("aspect_ratio", [state.size.0 as f32 / state.size.1 as f32]);
                shader_lib.ghost.set_matrix_uniform("rotationMatrix", *ghost_rotation.as_ref());
                ghost.draw(&shader_lib.ghost, (self.pos_x, self.pos_y), ghost_geo);
            });

            // copy distorted ghost geometry
            main_fb.draw_with(|_fb| {
                shader_lib.dispersion.bind();
                shader_lib
                    .dispersion
                    .set_float_uniform("res", [state.size.0 as f32 / 64.0, state.size.1 as f32 / 64.0]);
                shader_lib.dispersion.set_int_uniform("samples", [self.samples as i32]);
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
            shader_lib.flare.set_float_uniform("flare_position", [self.pos_x, self.pos_y]);
            shader_lib.flare.set_float_uniform("aspect_ratio", [state.size.0 as f32 / state.size.1 as f32]);
            shader_lib.flare.set_float_uniform("blades", [self.blades as f32]);
            self.flare.draw(&shader_lib.flare, &quad);
        });
    }

    pub fn set_position(&mut self, (pos_x, pos_y): (f32, f32)) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    }
}
