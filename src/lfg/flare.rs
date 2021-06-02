use crate::gl_wrapper::{
    geometry::{quad, Geometry},
    shader::Shader,
};

pub struct Flare {
    geometry: Geometry,
    pos_x: f32,
    pos_y: f32,
}

impl Flare {
    pub fn new() -> Self {
        Self {
            geometry: quad(),
            pos_x: 0.0,
            pos_y: 0.0,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        shader.set_float_uniform("flare_position", [self.pos_x, self.pos_y]);
        self.geometry.draw();
    }

    pub fn set_position(&mut self, new_x: f32, new_y: f32) {
        self.pos_x = new_x;
        self.pos_y = new_y;
    }
}
