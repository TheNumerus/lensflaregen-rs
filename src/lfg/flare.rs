use gl_wrapper::{geometry::Geometry, shader::Shader};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flare {
    pub color: [f32; 4],
    pub style: FlareStyle,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlareStyle {
    Normal,
    Anamorphic,
}
