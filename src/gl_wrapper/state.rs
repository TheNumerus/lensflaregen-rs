pub struct State {}

impl State {
    pub fn blend(blend: Blend) {
        unsafe {
            match blend {
                Blend::Enable(src, dst) => {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(src, dst);
                }
                Blend::Disable => gl::Disable(gl::BLEND),
            }
        }
    }

    pub fn viewport(x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            gl::Viewport(x as i32, y as i32, width as i32, height as i32);
        }
    }
}

// TODO remove GLenum
pub enum Blend {
    Enable(gl::types::GLenum, gl::types::GLenum),
    Disable,
}
