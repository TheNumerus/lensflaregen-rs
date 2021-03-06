use gl_wrapper::shader::{Shader, ShaderBuilder, ShaderCompilationError};

const COMMON_SHADER: &str = include_str!("../../shaders/common.glsl");

const QUAD_VERT: &str = include_str!("../../shaders/quad.vert");
const FLARE_FRAG: &str = include_str!("../../shaders/flare.frag");

const GHOST_VERT: &str = include_str!("../../shaders/ghost.vert");
const GHOST_FRAG: &str = include_str!("../../shaders/ghost.frag");

const TONEMAP: &str = include_str!("../../shaders/tonemap.frag");
const DISPERSION: &str = include_str!("../../shaders/dispersion_copy.frag");

pub struct ShaderLib {
    pub flare: Shader,
    pub flare_anam: Shader,
    pub ghost: Shader,
    pub dispersion: Shader,
    pub tonemap: Shader,
}

impl ShaderLib {
    pub fn new() -> Result<Self, ShaderCompilationError> {
        let flare = ShaderBuilder::new(QUAD_VERT, FLARE_FRAG).with_common_code(COMMON_SHADER).build()?;
        let flare_anam = ShaderBuilder::new(QUAD_VERT, FLARE_FRAG)
            .with_common_code(COMMON_SHADER)
            .with_define("ANAMORPHIC")
            .build()?;
        let ghost = ShaderBuilder::new(GHOST_VERT, GHOST_FRAG).with_common_code(COMMON_SHADER).build()?;
        let tonemap = ShaderBuilder::new(QUAD_VERT, TONEMAP).with_common_code(COMMON_SHADER).build()?;
        let dispersion = ShaderBuilder::new(QUAD_VERT, DISPERSION).with_common_code(COMMON_SHADER).build()?;

        let lib = Self {
            flare,
            flare_anam,
            ghost,
            dispersion,
            tonemap,
        };

        Ok(lib)
    }
}

impl Default for ShaderLib {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
