use cgmath::{prelude::*, vec2, vec3, Deg, Matrix2, Matrix4, Rad};

use crate::gl_wrapper::{
    geometry::{AttrSize, Geometry, GeometryBuilder, GeometryType},
    shader::Shader,
};

use super::effect::Effect;

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
        }
    }

    pub fn draw(&self, shader: &Shader, flare_pos: (f32, f32), geo: &Geometry, effect: &Effect) {
        shader.set_float_uniform("color", self.color);
        shader.set_float_uniform("empty", [self.center_transparency]);
        shader.set_float_uniform("ratio", [self.aspect_ratio]);

        let mut ghost_x = ((flare_pos.0 - 0.5) * 2.0) * self.offset;
        let mut ghost_y = ((flare_pos.1 - 0.5) * 2.0) * self.offset;

        let flare_vector = (vec2(flare_pos.0, flare_pos.1) - vec2(0.5, 0.5)).normalize();
        // add perpendicular offset
        ghost_x += flare_vector.y * self.perpendicular_offset;
        ghost_y += -flare_vector.x * self.perpendicular_offset;

        let model_m = Matrix4::from_translation(vec3(ghost_x, ghost_y, 0.0)) * Matrix4::from_scale(self.size / 100.0);
        shader.set_matrix_uniform("modelMatrix", *model_m.as_ref());
        shader.set_matrix_uniform("rotationMatrix", *Matrix4::from_angle_z(Rad(effect.rotation)).as_ref());

        geo.draw();
    }

    pub fn draw_dispersed(&self, shader: &Shader, quad: &Geometry) {
        shader.set_float_uniform("intensity", [self.intensity]);
        shader.set_float_uniform("dispersion", [self.dispersion]);
        shader.set_float_uniform("distortion", [self.distortion]);

        quad.draw();
    }
}

impl Default for Ghost {
    fn default() -> Self {
        Self::new()
    }
}

pub fn gen_ghost_geo(blades: u32) -> Geometry {
    let mut vert_data = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0];

    let mut start = cgmath::vec2(1.0, 0.0);

    for _ in 0..=blades {
        vert_data.extend_from_slice(&[start.x, start.y, 1.0, 1.0, 1.0, 1.0]);
        start = Matrix2::from_angle(Deg(360.0 / blades as f32)) * start;
    }

    GeometryBuilder::new(vert_data)
        .mode(GeometryType::TriangleFan)
        .with_attributes(&[AttrSize::Vec2, AttrSize::Vec4])
        .build()
}
