use crate::components::spatial::vec4::Vec4;
use crate::components::context::context;
use crate::components::render::stack::State::Blend;
use crate::components::wrapper::framebuffer::Framebuffer;
use crate::components::wrapper::shader::Shader;
use crate::gl_binds::gl11::{RGB, RGBA};
use crate::gl_binds::gl20::{ActiveTexture, TEXTURE0, TEXTURE1, TEXTURE2, TEXTURE3};
use crate::gl_binds::gl30::{BLEND, Disable, Enable};

/// Creates a mask based off of brightness in the mask framebuffer, applied onto the draw framebuffer
///
/// If the mask is fully white, then the draw framebuffer will show. If it's black it wont
///
/// Make sure ALL drawing within the mask and apply layer DO NOT HAVE BLEND ENABLED
pub struct FramebufferMask {
    mask_fb: u32,
    draw_fb: u32,
}

impl FramebufferMask {
    pub unsafe fn new() -> FramebufferMask {
        let window = context().window();

        FramebufferMask {
            mask_fb: context().fb_manager().create_fb(RGBA).unwrap(),
            draw_fb: context().fb_manager().create_fb(RGBA).unwrap(),
        }
    }

    unsafe fn mask_fb(&self) -> &mut Framebuffer {
        context().fb_manager().fb(self.mask_fb)
    }

    unsafe fn draw_fb(&self) -> &mut Framebuffer {
        context().fb_manager().fb(self.draw_fb)
    }

    /// Binds and clears the mask framebuffer to be drawn onto
    pub unsafe fn begin_mask(&mut self) {
        self.mask_fb().bind();
        Framebuffer::clear_current();
        Disable(BLEND);
    }

    pub unsafe fn end_mask(&self) {
        self.mask_fb().unbind();
        Enable(BLEND);
    }

    /// Binds and clears the apply framebuffer to be drawn onto
    pub unsafe fn begin_draw(&mut self) {
        self.draw_fb().bind();
        Framebuffer::clear_current();
        context().renderer().stack().push_l(Blend(false), 1);
    }

    pub unsafe fn end_draw(&self) {
        self.draw_fb().unbind();
        context().renderer().stack().pop();
    }

    // Applies to mask framebuffer to the draw framebuffer, and renders it onto the parent framebuffer
    pub unsafe fn render(&self) {
        // Disable(BLEND);
        let window = context().window();
        let renderer = context().renderer();
        renderer.mask_shader.bind();

        renderer.mask_shader.u_put_int("u_top_tex", vec![1]);
        renderer.mask_shader.u_put_int("u_mask_tex", vec![2]);
        renderer.mask_shader.u_put_int("u_bottom_tex", vec![3]);

        ActiveTexture(TEXTURE3);
        context().framebuffer().bind_texture();

        ActiveTexture(TEXTURE2);
        self.mask_fb().bind_texture();

        // Bind TEXTURE0 last so that it doesn't have to be set later
        ActiveTexture(TEXTURE1);
        self.draw_fb().bind_texture();

        ActiveTexture(TEXTURE0);

        renderer.draw_texture_rect(&Vec4::ltrb(0.0, window.height as f32, window.width as f32, 0.0), 0x00000000);

        Shader::unbind();
        // Enable(BLEND);
    }
}