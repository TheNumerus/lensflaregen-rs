use crate::gl_wrapper::{
    geometry::{quad, Geometry},
    shader::Shader,
};

pub struct Flare {
    geometry: Geometry,
    pos_x: f32,
    pos_y: f32,
    color: [f32; 4],
}

impl Flare {
    pub fn new() -> Self {
        Self {
            geometry: quad(),
            pos_x: 0.0,
            pos_y: 0.0,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        shader.set_float_uniform("flare_position", [self.pos_x, self.pos_y]);
        shader.set_float_uniform("color", self.color);
        self.geometry.draw();
    }

    pub fn set_position(&mut self, new_x: f32, new_y: f32) {
        self.pos_x = new_x;
        self.pos_y = new_y;
    }

    pub fn set_color(&mut self, color: &[f32; 4]) {
        self.color = *color;
    }
}
