use std::{convert::TryFrom, num::NonZeroU8};

use crate::window_state::WindowState;
use cgmath::{Matrix2, Matrix4, Rad};
use gl_wrapper::{framebuffer::Framebuffer, geometry::Geometry};

use super::{flare::Flare, ghost::Ghost, shader_lib::ShaderLib, LfgError};

pub struct Effect {
    pub flare: Flare,
    pub ghosts: Vec<Ghost>,
    pub rotation: f32,
    pub aperture_shape: ApertureShape,
    pub pos_x: f32,
    pub pos_y: f32,
    pub samples: u16,
    pub tonemap: bool,
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
            aperture_shape: ApertureShape::from_blade_count(8).unwrap(),
            pos_x: 0.8,
            pos_y: 0.8,
            samples: 8,
            tonemap: true,
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
                ghost.draw(&shader_lib.ghost, state, (self.pos_x, self.pos_y), ghost_geo);
            });

            // copy distorted ghost geometry
            main_fb.draw_with(|_fb| {
                shader_lib.dispersion.bind();
                shader_lib
                    .dispersion
                    .set_float_uniform("res", [state.size.0 as f32 / 128.0, state.size.1 as f32 / 128.0]);
                shader_lib.dispersion.set_int_uniform("samples", [self.samples as i32]);
                side_fb.bind_as_color_texture(0);

                ghost.draw_dispersed(&shader_lib.dispersion, state, (self.pos_x, self.pos_y), &quad);
            });
        }

        // render flare on top
        main_fb.draw_with(|_fb| {
            let shader = match self.flare.style {
                super::flare::FlareStyle::Normal => &shader_lib.flare,
                super::flare::FlareStyle::Anamorphic => &shader_lib.flare_anam,
            };

            shader.bind();
            shader.set_float_uniform("flare_position", [self.pos_x, self.pos_y]);
            shader.set_float_uniform("aspect_ratio", [state.size.0 as f32 / state.size.1 as f32]);
            shader.set_float_uniform("blades", [self.aperture_shape.get_blade_count() as f32]);

            shader.set_matrix_uniform("texture_rotation", *Matrix2::from_angle(Rad(self.rotation)).as_ref());
            self.flare.draw(shader, &quad);
        });
    }

    pub fn set_position(&mut self, (pos_x, pos_y): (f32, f32)) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ApertureShape {
    Polygonal(NonZeroU8),
    Circular,
}

impl ApertureShape {
    pub fn get_blade_count(&self) -> u8 {
        match self {
            ApertureShape::Polygonal(b) => b.get(),
            ApertureShape::Circular => 255,
        }
    }

    pub fn from_blade_count(value: u8) -> Result<Self, LfgError> {
        Self::try_from(value)
    }
}

impl TryFrom<u8> for ApertureShape {
    type Error = LfgError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ApertureShape::Circular),
            1 | 2 => Err(LfgError::InvalidEffectValue("aperture shape".into())),
            _ => Ok(ApertureShape::Polygonal(NonZeroU8::new(value).unwrap())),
        }
    }
}
