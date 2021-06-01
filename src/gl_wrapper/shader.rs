use std::{
    ffi::{CStr, CString},
    ptr,
};

use gl::types::GLenum;

pub struct Shader {
    program_id: u32,
}

impl Shader {
    pub fn from_str(vert: &str, frag: &str) -> Result<Self, String> {
        unsafe {
            let vert_id = compile_shader(ShaderType::Vertex, vert)?;
            let frag_id = compile_shader(ShaderType::Fragment, frag)?;

            let mut success = 0;
            let mut info_log = [0_i8; 512];
            let mut info_len = 512;

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vert_id);
            gl::AttachShader(program_id, frag_id);
            gl::LinkProgram(program_id);
            // check for linking errors
            gl::GetProgramiv(program_id, gl::LINK_STATUS, ptr::addr_of_mut!(success));
            if success != 1 {
                gl::GetProgramInfoLog(program_id, 512, ptr::addr_of_mut!(info_len), info_log.as_mut_ptr());
                let msg = CStr::from_ptr(info_log[0..(info_len as usize + 1)].as_mut_ptr());
                return Err(msg.to_string_lossy().to_string());
            }

            gl::DeleteShader(vert_id);
            gl::DeleteShader(frag_id);

            Ok(Self { program_id })
        }
    }

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

enum ShaderType {
    Fragment,
    Vertex,
}

impl From<ShaderType> for GLenum {
    fn from(st: ShaderType) -> Self {
        match st {
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Vertex => gl::VERTEX_SHADER,
        }
    }
}

unsafe fn compile_shader(shader_type: ShaderType, src: &str) -> Result<u32, String> {
    //
    let src_cstr = CString::new(src).unwrap();
    let src_ptr = src_cstr.as_ptr();
    let shader_id = gl::CreateShader(shader_type.into());

    gl::ShaderSource(shader_id, 1, ptr::addr_of!(src_ptr), ptr::null());
    gl::CompileShader(shader_id);

    let mut success = 0;
    let mut info_log = [0_i8; 512];
    let mut info_len = 0;
    gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, ptr::addr_of_mut!(success));

    if success != 1 {
        gl::GetShaderInfoLog(shader_id, 512, ptr::addr_of_mut!(info_len), info_log.as_mut_ptr());
        let msg = CStr::from_ptr(info_log[0..(info_len as usize + 1)].as_mut_ptr());
        return Err(msg.to_string_lossy().to_string());
    }

    Ok(shader_id)
}
