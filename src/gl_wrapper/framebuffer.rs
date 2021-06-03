use std::ptr;

use log::{debug, error};

pub struct Framebuffer {
    fb_id: u32,
    color_buf: u32,
}

impl Framebuffer {
    pub fn hdr(width: u32, height: u32) -> Self {
        unsafe {
            let mut fb_id = 0;
            gl::GenFramebuffers(1, ptr::addr_of_mut!(fb_id));

            let mut color_buf = 0;
            gl::GenTextures(1, ptr::addr_of_mut!(color_buf));

            gl::BindTexture(gl::TEXTURE_2D, color_buf);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::R11F_G11F_B10F as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::FLOAT,
                ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindFramebuffer(gl::FRAMEBUFFER, fb_id);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, color_buf, 0);
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                error!("Framebuffer not complete");
                panic!();
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            debug!("Framebuffer {} generated", fb_id);
            Self { fb_id, color_buf }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fb_id);
        }
    }

    pub fn bind_as_color_texture(&self, unit: u8) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit as u32);
            gl::BindTexture(gl::TEXTURE_2D, self.color_buf);
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::R11F_G11F_B10F as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::FLOAT,
                ptr::null(),
            );
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, self.fb_id as *const _);
            gl::DeleteTextures(1, self.color_buf as *const _);
        }
    }
}
