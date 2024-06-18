use std::path;
use gl::TEXTURE2;
use crate::components::render::bounds::Bounds;
use crate::components::window::Window;
use crate::components::wrapper::framebuffer::Framebuffer;
use crate::gl_binds::gl30::{ActiveTexture, BLEND, BlendFunc, Enable, ONE_MINUS_SRC_ALPHA, RGBA, SRC_ALPHA, TEXTURE0, TEXTURE1};

/// Creates a mask based off of brightness in the mask framebuffer, applied onto the draw framebuffer
///
/// If the mask is fully white, then the draw framebuffer will show. If it's black it wont
pub struct FramebufferMask {
    mask_framebuffer: Framebuffer,
    apply_framebuffer: Framebuffer,
}

impl FramebufferMask {
    pub unsafe fn new(window: &mut Window) -> FramebufferMask {
        FramebufferMask {
            mask_framebuffer: Framebuffer::new(RGBA, window.width, window.height).expect("Failed to make mask framebuffer"),
            apply_framebuffer: Framebuffer::new(RGBA, window.width, window.height).expect("Failed to make apply framebuffer"),
        }
    }

    /// Binds and clears the mask framebuffer to be drawn onto
    pub unsafe fn begin_mask(&mut self) {
        self.mask_framebuffer.bind();
        self.mask_framebuffer.clear();
    }

    pub unsafe fn end_mask(&self) {
        self.mask_framebuffer.unbind();
    }

    /// Binds and clears the apply framebuffer to be drawn onto
    pub unsafe fn begin_draw(&mut self) {
        self.apply_framebuffer.bind();
        self.apply_framebuffer.clear();
    }

    pub unsafe fn end_draw(&self) {
        self.apply_framebuffer.unbind();
    }

    /// Applies to mask framebuffer to the draw framebuffer, and renders it onto the parent framebuffer
    pub unsafe fn render(&self, window: &mut Window) {
        window.renderer.color_mult_shader.bind();

        window.renderer.color_mult_shader.u_put_int("draw_texture", vec![0]);
        window.renderer.color_mult_shader.u_put_int("mask_texture", vec![1]);
        // window.renderer.color_mult_shader.u_put_int("base_texture", vec![2]);

        // ActiveTexture(TEXTURE2);
        // window.framebuffer().bind_texture();

        ActiveTexture(TEXTURE1);
        self.mask_framebuffer.bind_texture();

        // Bind TEXTURE0 last so that it doesn't have to be set later
        ActiveTexture(TEXTURE0);
        self.apply_framebuffer.bind_texture();

        window.renderer.draw_texture_rect(&Bounds::from_ltrb(0.0, window.height as f32, window.width as f32, 0.0),0x00000000);

        window.renderer.color_mult_shader.unbind();
    }
}