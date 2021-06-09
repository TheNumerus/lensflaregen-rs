use std::rc::Rc;

use cgmath::{prelude::*, Deg, Matrix2, Matrix4};

use crate::gl_wrapper::{
    geometry::{AttrSize, Geometry, GeometryBuilder, GeometryType},
    shader::Shader,
};

pub struct Ghost {
    color: [f32; 4],
    geometry: Rc<Geometry>,
    center_transparency: f32,
    aspect_ratio: f32,
}

impl Ghost {
    pub fn new(geo: Rc<Geometry>) -> Self {
        Ghost {
            color: [0.5, 0.5, 0.5, 1.0],
            geometry: geo,
            center_transparency: 1.0,
            aspect_ratio: 1.0,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        shader.set_float_uniform("color", self.color);
        shader.set_float_uniform("empty", [self.center_transparency]);
        shader.set_float_uniform("ratio", [self.aspect_ratio]);
        shader.set_matrix_uniform("modelMatrix", *Matrix4::from_scale(0.4).as_ref());
        shader.set_matrix_uniform("rotationMatrix", *Matrix4::identity().as_ref());

        self.geometry.draw();
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
