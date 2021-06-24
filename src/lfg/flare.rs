use crate::gl_wrapper::{geometry::Geometry, shader::Shader};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flare {
    color: [f32; 4],
    style: FlareStyle,
}

impl Flare {
    pub fn new() -> Self {
        Self {
            color: [1.0, 0.5, 0.5, 1.0],
            style: FlareStyle::Normal,
        }
    }

    pub fn draw(&self, shader: &Shader, quad: &Geometry) {
        shader.set_float_uniform("color", self.color);
        quad.draw();
    }
    pub fn set_color(&mut self, color: &[f32; 4]) {
        self.color = *color;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlareStyle {
    Normal,
    Anamorphic,
}
