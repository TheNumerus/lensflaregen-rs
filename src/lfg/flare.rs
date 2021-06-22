use crate::gl_wrapper::{geometry::Geometry, shader::Shader};

pub struct Flare {
    pos_x: f32,
    pos_y: f32,
    color: [f32; 4],
}

impl Flare {
    pub fn new() -> Self {
        Self {
            pos_x: 0.0,
            pos_y: 0.0,
            color: [1.0, 0.5, 0.5, 1.0],
        }
    }

    pub fn draw(&self, shader: &Shader, quad: &Geometry) {
        shader.set_float_uniform("color", self.color);
        quad.draw();
    }

    pub fn set_position(&mut self, new_x: f32, new_y: f32) {
        self.pos_x = new_x;
        self.pos_y = new_y;
    }

    pub fn set_color(&mut self, color: &[f32; 4]) {
        self.color = *color;
    }
}
