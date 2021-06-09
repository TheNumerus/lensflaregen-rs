use std::{
    ffi::{CStr, CString},
    fmt::Display,
    ptr,
};

use gl::types::GLenum;
use log::{debug, error};
use thiserror::Error;

pub struct Shader {
    program_id: u32,
}

const MAX_ERR_LEN: i32 = 1024;
const SHADER_VERSION: &str = "#version 450\n";

impl Shader {
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn set_float_uniform<const N: usize>(&self, name: &str, float_vec: [f32; N]) {
        unsafe {
            let loc = self.get_uniform_location(name).unwrap();

            match N {
                1 => gl::Uniform1f(loc, float_vec[0]),
                2 => gl::Uniform2f(loc, float_vec[0], float_vec[1]),
                3 => gl::Uniform3f(loc, float_vec[0], float_vec[1], float_vec[2]),
                4 => gl::Uniform4f(loc, float_vec[0], float_vec[1], float_vec[2], float_vec[3]),
                _ => panic!("invalid float vector size passed"),
            }
        };
    }

    pub fn set_int_uniform<const N: usize>(&self, name: &str, int_vec: [i32; N]) {
        unsafe {
            let loc = self.get_uniform_location(name).unwrap();

            match N {
                1 => gl::Uniform1i(loc, int_vec[0]),
                2 => gl::Uniform2i(loc, int_vec[0], int_vec[1]),
                3 => gl::Uniform3i(loc, int_vec[0], int_vec[1], int_vec[2]),
                4 => gl::Uniform4i(loc, int_vec[0], int_vec[1], int_vec[2], int_vec[3]),
                _ => panic!("invalid float vector size passed"),
            }
        };
    }

    pub fn set_matrix_uniform<const N: usize>(&self, name: &str, matrix_vec: [f32; N]) {
        unsafe {
            let loc = self.get_uniform_location(name).unwrap();

            match N {
                4 => gl::UniformMatrix2fv(loc, 1, false as u8, matrix_vec.as_ptr()),
                9 => gl::UniformMatrix3fv(loc, 1, false as u8, matrix_vec.as_ptr()),
                16 => gl::UniformMatrix4fv(loc, 1, false as u8, matrix_vec.as_ptr()),
                _ => panic!("invalid matrix size passed"),
            }
        };
    }

    fn get_uniform_location(&self, name: &str) -> Result<i32, String> {
        let name_cstr = CString::new(name).map_err(|_| "uniform name has zero bytes")?;
        let name_ptr = name_cstr.as_ptr();

        let loc;
        unsafe {
            // `program_id` should be valid if `Self` if properly constructed
            loc = gl::GetUniformLocation(self.program_id, name_ptr);
        }
        if loc == -1 {
            return Err(format!("uniform with name '{}' does not exist", name));
        }
        Ok(loc)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Fragment,
    Vertex,
}

impl Display for ShaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderType::Fragment => f.write_str("Fragment"),
            ShaderType::Vertex => f.write_str("Vertex"),
        }
    }
}

impl From<ShaderType> for GLenum {
    fn from(st: ShaderType) -> Self {
        match st {
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Vertex => gl::VERTEX_SHADER,
        }
    }
}

unsafe fn compile_shader(shader_type: ShaderType, src: &str, includes: &[&str]) -> Result<u32, ShaderCompilationError> {
    let shader_id = gl::CreateShader(shader_type.into());

    let mut sources = Vec::with_capacity(includes.len() + 1);
    let mut pointers = Vec::with_capacity(includes.len() + 1);

    for inc in includes {
        sources.push(CString::new(*inc).unwrap());
    }

    let src_cstr = CString::new(src).unwrap();
    sources.push(src_cstr);

    for c_str in &sources {
        pointers.push(c_str.as_ptr());
    }

    gl::ShaderSource(shader_id, sources.len() as i32, pointers.as_ptr(), ptr::null());
    gl::CompileShader(shader_id);

    let mut success = 0;
    let mut info_log = [0_i8; MAX_ERR_LEN as usize];
    let mut info_len = 0;
    gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, ptr::addr_of_mut!(success));

    if success != 1 {
        gl::GetShaderInfoLog(shader_id, MAX_ERR_LEN, ptr::addr_of_mut!(info_len), info_log.as_mut_ptr());
        let msg = CStr::from_ptr(info_log[0..(info_len as usize + 1)].as_mut_ptr());
        let msg = snailquote::unescape(&msg.to_string_lossy()).unwrap();
        let e = ShaderCompilationError::ProgramError(shader_type, msg);
        error!("{} shader compilation error", shader_type);
        return Err(e);
    }

    Ok(shader_id)
}

#[derive(Error, Debug)]
pub enum ShaderCompilationError {
    #[error("{0} shader program error: \n{1}")]
    ProgramError(ShaderType, String),
    #[error("Shader linking error: \n{0}")]
    LinkageError(String),
}

pub struct ShaderBuilder<'a> {
    vert: &'a str,
    frag: &'a str,
    includes: Vec<&'a str>,
}

impl<'a> ShaderBuilder<'a> {
    pub fn new(vert: &'a str, frag: &'a str) -> Self {
        Self {
            vert,
            frag,
            includes: vec![SHADER_VERSION],
        }
    }

    pub fn with_common_code(&mut self, include: &'a str) -> &mut Self {
        self.includes.push(include);
        self
    }

    pub fn build(&self) -> Result<Shader, ShaderCompilationError> {
        unsafe {
            let vert_id = compile_shader(ShaderType::Vertex, self.vert, &self.includes)?;
            let frag_id = compile_shader(ShaderType::Fragment, self.frag, &self.includes)?;

            let mut success = 0;
            let mut info_log = [0_i8; MAX_ERR_LEN as usize];
            let mut info_len = 0;

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vert_id);
            gl::AttachShader(program_id, frag_id);
            gl::LinkProgram(program_id);
            // check for linking errors
            gl::GetProgramiv(program_id, gl::LINK_STATUS, ptr::addr_of_mut!(success));
            if success != 1 {
                gl::GetProgramInfoLog(program_id, MAX_ERR_LEN, ptr::addr_of_mut!(info_len), info_log.as_mut_ptr());
                let msg = CStr::from_ptr(info_log[0..(info_len as usize + 1)].as_mut_ptr());
                let msg = snailquote::unescape(&msg.to_string_lossy()).unwrap();
                let e = ShaderCompilationError::LinkageError(msg);
                error!("Shader linking error");
                return Err(e);
            }

            gl::DeleteShader(vert_id);
            gl::DeleteShader(frag_id);

            debug!("Shader program {} constructed", program_id);

            Ok(Shader { program_id })
        }
    }
}
