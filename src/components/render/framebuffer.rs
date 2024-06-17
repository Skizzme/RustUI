use std::ptr::null;
use crate::components::window::Window;
use crate::gl_binds::gl30::*;
use crate::gl_binds::gl30::types::{GLenum, GLint};

pub struct Framebuffer {
    framebuffer_id: u32,
    texture_id: u32,
    parent_framebuffer: u32,
}

impl Framebuffer {
    pub unsafe fn new(format: GLenum, window_width: i32, window_height: i32, parent: u32) -> Result<Framebuffer, String> {
        let mut framebuffer = 0u32;
        GenFramebuffers(1, &mut framebuffer);
        BindFramebuffer(FRAMEBUFFER, framebuffer);

        let mut tex = 0;
        GenTextures(1, &mut tex);
        BindTexture(TEXTURE_2D, tex);

        TexImage2D(TEXTURE_2D, 0, format as GLint, window_width, window_height, 0, format, UNSIGNED_BYTE, null());

        TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);

        FramebufferTexture2D(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, tex, 0);

        let status = CheckFramebufferStatus(FRAMEBUFFER);
        BindFramebuffer(FRAMEBUFFER, 0);
        if (status != FRAMEBUFFER_COMPLETE) {
            Err(format!("Failed to create framebuffer object. Status: {}", status))
        } else {
            Ok(Framebuffer { framebuffer_id: framebuffer, texture_id: tex, parent_framebuffer: parent })
        }
    }

    pub unsafe fn bind(&self) {
        BindFramebuffer(FRAMEBUFFER, self.framebuffer_id);
    }

    pub unsafe fn clear(&self) {
        Clear(COLOR_BUFFER_BIT);
        ClearColor(0.0, 0.0, 0.0, 0.0);
        Clear(DEPTH_BUFFER_BIT);
        Clear(STENCIL_BUFFER_BIT);
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    }

    pub unsafe fn unbind(&self) {
        BindFramebuffer(FRAMEBUFFER, self.parent_framebuffer);
    }

    pub unsafe fn bind_texture(&self) {
        BindTexture(TEXTURE_2D, self.texture_id);
    }

    pub unsafe fn unbind_texture(&self) {
        BindTexture(TEXTURE_2D, 0);
    }

    pub unsafe fn delete(&self) {
        DeleteFramebuffers(1, &self.framebuffer_id);
        DeleteTextures(1, &self.texture_id);
    }

    pub fn texture_id(&self) -> u32 {
        self.texture_id
    }

    pub fn id(&self) -> u32 {
        self.framebuffer_id
    }
}