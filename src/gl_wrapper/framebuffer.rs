use std::ptr;

use log::{debug, error};

pub struct Framebuffer {
    fb_id: u32,
    color_buf: u32,
    bound: bool,
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
            Self {
                fb_id,
                color_buf,
                bound: false,
            }
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
            gl::BindTexture(gl::TEXTURE_2D, self.color_buf);
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
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn clear(&self) {
        if self.bound {
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
        }
    }

    pub fn blend(&self, blend: Blend) {
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

    pub fn draw_with<F: Fn(&Self)>(&mut self, draw: F) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fb_id);
        }
        self.bound = true;

        draw(self);

        self.bound = false;

        Self::bind_default();
    }

    pub fn draw_with_default<F: Fn(&Self)>(draw: F) {
        Self::bind_default();
        let dummy_fb = Self {
            bound: true,
            color_buf: 0,
            fb_id: 0,
        };
        draw(&dummy_fb);

        // don't run drop on dummy
        // gl::DeleteTextures and gl::DeleteFramebuffers should ignore zeroes, but without forget, this segfaults
        std::mem::forget(dummy_fb);
    }

    pub fn bind_default() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
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

// TODO remove GLenum
pub enum Blend {
    Enable(gl::types::GLenum, gl::types::GLenum),
    Disable,
}
