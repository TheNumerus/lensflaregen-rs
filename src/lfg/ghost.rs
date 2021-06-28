use cgmath::{prelude::*, vec2, Deg, Matrix2, Matrix4, Vector2};

use gl_wrapper::{
    geometry::{AttrSize, Geometry, GeometryBuilder, GeometryType},
    shader::Shader,
};

use crate::window_state::WindowState;

pub struct Ghost {
    pub color: [f32; 4],
    pub offset: f32,
    pub perpendicular_offset: f32,
    pub size: f32,
    pub dispersion: f32,
    pub distortion: f32,
    pub intensity: f32,
    pub center_transparency: f32,
    pub aspect_ratio: f32,
    pub dispersion_center: DispersionCenter,
}

impl Ghost {
    pub fn new() -> Self {
        Ghost {
            color: [0.5, 0.5, 0.5, 1.0],
            offset: -1.0,
            perpendicular_offset: 0.0,
            size: 30.0,
            intensity: 2.0,
            dispersion: 0.1,
            distortion: 0.9,
            center_transparency: 1.0,
            aspect_ratio: 1.0,
            dispersion_center: DispersionCenter::Image,
        }
    }

    pub fn draw(&self, shader: &Shader, _state: &WindowState, flare_pos: (f32, f32), geo: &Geometry) {
        shader.set_float_uniform("color", self.color);
        shader.set_float_uniform("empty", [self.center_transparency]);
        shader.set_float_uniform("ratio", [self.aspect_ratio]);

        let ghost_pos = self.ghost_pos_from_flare_pos(flare_pos);

        let model_m = Matrix4::from_translation(ghost_pos.extend(0.0)) * Matrix4::from_scale(self.size / 100.0);
        shader.set_matrix_uniform("modelMatrix", *model_m.as_ref());

        geo.draw();
    }

    pub fn draw_dispersed(&self, shader: &Shader, state: &WindowState, flare_pos: (f32, f32), quad: &Geometry) {
        shader.set_float_uniform("intensity", [self.intensity]);
        shader.set_float_uniform("dispersion", [self.dispersion]);
        shader.set_float_uniform("distortion", [self.distortion]);

        let center = match self.dispersion_center {
            DispersionCenter::Ghost => true,
            DispersionCenter::Image => false,
        };

        let jitter_offset = match state.frame_num % 4 {
            0 => [0.0, 0.0],
            1 => [0.5, 0.0],
            2 => [0.5, 0.5],
            3 => [0.0, 0.5],
            _ => [0.0, 0.0],
        };

        shader.set_int_uniform("disperse_from_ghost_center", [center as i32]);
        shader.set_float_uniform("jitter_offset", jitter_offset);
        if center {
            let ghost_pos = self.ghost_pos_from_flare_pos(flare_pos);
            shader.set_float_uniform("ghost_pos", [ghost_pos.x, ghost_pos.y]);
        }

        quad.draw();
    }

    fn ghost_pos_from_flare_pos(&self, flare_pos: (f32, f32)) -> Vector2<f32> {
        let flare_vec = Vector2::from(flare_pos);

        // map from <0.0; 1.0> to <-1.0; 1.0>
        let mut ghost_pos = (flare_vec * 2.0 - Vector2::from_value(1.0)) * self.offset;

        // mapped to direction vector from image center
        let flare_vec_mapped = (flare_vec - vec2(0.5, 0.5)).normalize();

        // add perpendicular offset
        ghost_pos.x += flare_vec_mapped.y * self.perpendicular_offset;
        ghost_pos.y += -flare_vec_mapped.x * self.perpendicular_offset;

        ghost_pos
    }
}

impl Default for Ghost {
    fn default() -> Self {
        Self::new()
    }
}

pub fn gen_ghost_geo(blades: u32) -> Geometry {
    let mut vert_data = Vec::with_capacity((blades as usize + 2) * 3);
    vert_data.extend_from_slice(&[0.0, 0.0, 0.0]);

    let mut start = cgmath::vec2(1.0, 0.0);

    for _ in 0..=blades {
        vert_data.extend_from_slice(&[start.x, start.y, 1.0]);
        start = Matrix2::from_angle(Deg(360.0 / blades as f32)) * start;
    }

    GeometryBuilder::new(vert_data)
        .mode(GeometryType::TriangleFan)
        .with_attributes(&[AttrSize::Vec2, AttrSize::Float])
        .build()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DispersionCenter {
    Ghost,
    Image,
}
