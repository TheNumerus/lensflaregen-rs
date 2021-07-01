use gl_wrapper::{geometry::Geometry, shader::Shader};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flare {
    pub color: [f32; 4],
    pub intensity: f32,
    pub size: f32,
    pub ray_intensity: f32,
    pub style: FlareStyle,
}

impl Flare {
    pub fn new() -> Self {
        Self {
            color: [1.0, 0.5, 0.5, 1.0],
            intensity: 1.0,
            ray_intensity: 1.0,
            size: 5.0,
            style: FlareStyle::Normal,
        }
    }

    pub fn draw(&self, shader: &Shader, quad: &Geometry) {
        shader.set_float_uniform("color", self.color);
        shader.set_float_uniform("intensity", [self.intensity]);
        shader.set_float_uniform("size", [self.size]);
        shader.set_float_uniform("ray_intensity", [self.ray_intensity]);

        quad.draw();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlareStyle {
    Normal,
    Anamorphic,
}
