use crate::{
    geometry::{quad, Geometry},
    shader::Shader,
};

pub struct Flare {
    geometry: Geometry,
}

impl Flare {
    pub fn new() -> Self {
        Self { geometry: quad() }
    }

    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.geometry.draw();
    }
}
