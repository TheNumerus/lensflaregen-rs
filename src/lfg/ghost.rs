use crate::gl_wrapper::{
    geometry::{AttrSize, Geometry, GeometryBuilder, GeometryType},
    shader::Shader,
};

const VERTICES: [f32; 15] = [
    -0.5, -0.5, 1.0, 0.0, 0.0, //
    0.5, -0.5, 0.0, 1.0, 0.0, //
    0.0, 0.5, 0.0, 0.0, 1.0,
];

pub struct Ghost {
    color: [f32; 4],
    geometry: Geometry,
}

impl Ghost {
    pub fn new() -> Self {
        let geometry = GeometryBuilder::new(VERTICES.to_vec())
            .mode(GeometryType::Triangles)
            .with_attributes(&[AttrSize::Vec2, AttrSize::Vec3])
            .build();

        Ghost {
            color: [1.0, 1.0, 1.0, 1.0],
            geometry,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        shader.set_float_uniform("color", self.color);

        self.geometry.draw();
    }
}
