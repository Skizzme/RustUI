use std::collections::HashMap;
use std::ptr::null;

use crate::components::context::context;
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::gl30::*;
use crate::gl_binds::gl30::types::{GLenum, GLint};

pub struct FramebufferManager {
    framebuffers: HashMap<u32, Framebuffer>,
}

impl FramebufferManager {
    pub fn new() ->  Self{
        FramebufferManager {
            framebuffers: HashMap::new(),
        }
    }

    pub unsafe fn resize(&mut self, width: i32, height: i32) {
        for fb in &mut self.framebuffers.values_mut() {
            fb.resize(width, height);
        }
    }

    pub unsafe fn fb(&mut self, id: u32) -> &mut Framebuffer {
        self.framebuffers.get_mut(&id).unwrap()
    }

    pub unsafe fn create_fb(&mut self, format: GLenum) -> Option<u32> {
        let window = context().window();
        self.create_fb_dims(format, window.width, window.height)
    }

    pub unsafe fn create_fb_dims(&mut self, format: GLenum, width: i32, height: i32,) -> Option<u32> {
        match Framebuffer::new(format, width, height) {
            Ok(fb) => {
                let id = fb.id();
                self.framebuffers.insert(id, fb);
                Some(id)
            }
            Err(err) => {
                println!("Failed to create framebuffer: {}", err);
                None
            }
        }
    }
}

pub struct Framebuffer {
    framebuffer_id: u32,
    texture_id: u32,
    parent_framebuffer: i32,
    width: i32,
    height: i32,
    format: GLenum,
}

impl Framebuffer {
    pub unsafe fn new(format: GLenum, window_width: i32, window_height: i32) -> Result<Framebuffer, String> {
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
        if status != FRAMEBUFFER_COMPLETE {
            Err(format!("Failed to create framebuffer object. Status: {}", status))
        } else {
            Ok(Framebuffer { framebuffer_id: framebuffer, texture_id: tex, parent_framebuffer: 0, width: window_width, height: window_height, format, })
        }
    }

    pub unsafe fn bind(&mut self) -> (i32, i32) {
        GetIntegerv(FRAMEBUFFER_BINDING, &mut self.parent_framebuffer);
        let mut parent_tex = 0i32;
        if self.parent_framebuffer != 0 {
            GetFramebufferAttachmentParameteriv(FRAMEBUFFER, COLOR_ATTACHMENT0, FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, &mut parent_tex);
        }
        // context::check_error(format!("bind to {}", self.parent_framebuffer).as_str());
        BindFramebuffer(FRAMEBUFFER, self.framebuffer_id);
        (self.parent_framebuffer, parent_tex)
    }

    pub unsafe fn tex_filter(&mut self, mode: GLenum) {
        BindTexture(TEXTURE_2D, self.texture_id);

        TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, mode as GLint);
        TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, mode as GLint);
        TexParameterf(TEXTURE_2D, TEXTURE_WRAP_S, 10496.0);
        TexParameterf(TEXTURE_2D, TEXTURE_WRAP_T, 10496.0);

        Texture::unbind();
    }

    pub unsafe fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.bind_texture();
        TexImage2D(TEXTURE_2D, 0, self.format as GLint, width, height, 0, self.format, UNSIGNED_BYTE, null());
        self.unbind_texture();
    }

    pub unsafe fn clear_current() {
        ClearColor(0.0, 0.0, 0.0, 0.0);
        Clear(COLOR_BUFFER_BIT);
        Clear(DEPTH_BUFFER_BIT);
        Clear(STENCIL_BUFFER_BIT);
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    }

    pub unsafe fn unbind(&self) {
        BindFramebuffer(FRAMEBUFFER, self.parent_framebuffer as u32);
    }

    pub unsafe fn bind_texture(&self) {
        BindTexture(TEXTURE_2D, self.texture_id);
    }

    pub unsafe fn unbind_texture(&self) {
        BindTexture(TEXTURE_2D, 0);
    }

    /// Copy this framebuffer to target, leaving the target framebuffer bound
    pub unsafe fn copy_bind(&self, target_fb: u32, target_tex: u32) {
        // Finish();
        // let st = Instant::now();
        Disable(BLEND);
        BindFramebuffer(FRAMEBUFFER, target_fb);

        context().renderer().blend_shader.bind();
        context().renderer().blend_shader.u_put_int("u_bottom_tex", vec![2]);
        context().renderer().blend_shader.u_put_int("u_top_tex", vec![1]);

        ActiveTexture(TEXTURE2);
        BindTexture(TEXTURE_2D, target_tex);

        ActiveTexture(TEXTURE1);
        BindTexture(TEXTURE_2D, self.texture_id);

        ActiveTexture(TEXTURE0);

        Texture::unbind();
        context().renderer().draw_screen_rect_flipped();
        Shader::unbind();
        Enable(BLEND);

        // Finish();
        // println!("cop {:?}", st.elapsed());
    }

    pub unsafe fn copy_raw(&self, target_fb: u32) {
        BindFramebuffer(READ_FRAMEBUFFER, self.framebuffer_id);
        BindFramebuffer(DRAW_FRAMEBUFFER, target_fb);
        BlitFramebuffer(0, 0, self.width, self.height, 0, 0, self.width, self.height, COLOR_BUFFER_BIT, NEAREST);
        BindFramebuffer(FRAMEBUFFER, target_fb);
    }

    pub unsafe fn copy_from_parent(&self) {
        BindFramebuffer(READ_FRAMEBUFFER, self.parent_framebuffer as u32);
        BindFramebuffer(DRAW_FRAMEBUFFER, self.framebuffer_id);
        BlitFramebuffer(0, 0, self.width, self.height, 0, 0, self.width, self.height, COLOR_BUFFER_BIT, NEAREST);
        BindFramebuffer(FRAMEBUFFER, self.framebuffer_id);
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