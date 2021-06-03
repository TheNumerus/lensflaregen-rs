use crate::gl_wrapper::shader::{Shader, ShaderCompilationError};

const COMMON_SHADER: &str = include_str!("../../shaders/common.glsl");

const QUAD_VERT: &str = include_str!("../../shaders/quad.vert");
const FLARE_FRAG: &str = include_str!("../../shaders/flare.frag");

const GHOST_VERT: &str = include_str!("../../shaders/ghost.vert");
const GHOST_FRAG: &str = include_str!("../../shaders/ghost.frag");

const TONEMAP: &str = include_str!("../../shaders/tonemap.frag");

pub struct ShaderLib {
    pub flare: Shader,
    pub ghost: Shader,
    pub tonemap: Shader,
}

impl ShaderLib {
    pub fn new() -> Result<Self, ShaderCompilationError> {
        let flare = Shader::from_str(QUAD_VERT, FLARE_FRAG)?;
        let ghost = Shader::from_str(GHOST_VERT, FLARE_FRAG)?;
        let tonemap = Shader::from_str(QUAD_VERT, TONEMAP)?;

        let lib = Self { flare, ghost, tonemap };

        Ok(lib)
    }
}

impl Default for ShaderLib {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
