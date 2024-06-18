use crate::components::render::bounds::Bounds;
use crate::components::render::framebuffer::Framebuffer;
use crate::components::window::Window;
use crate::gl_binds::gl30::{ActiveTexture, RGBA, TEXTURE0, TEXTURE1};

/// Uses the main window framebuffer and a new framebuffer and multiplies them together, mostly used for masking
pub struct FramebufferMask {
    mask_framebuffer: Framebuffer,
    apply_framebuffer: Framebuffer,
}

impl FramebufferMask {
    pub unsafe fn new(window: &mut Window) -> FramebufferMask {
        FramebufferMask {
            mask_framebuffer: Framebuffer::new(RGBA, window.width, window.height, window.framebuffer().id()).expect("Failed to make mask framebuffer"),
            apply_framebuffer: Framebuffer::new(RGBA, window.width, window.height, window.framebuffer().id()).expect("Failed to make apply framebuffer"),
        }
    }

    /// Binds and clears the mask framebuffer to be drawn onto
    pub unsafe fn begin_mask(&self) {
        self.mask_framebuffer.bind();
        self.mask_framebuffer.clear();
    }

    pub unsafe fn end_mask(&self) {
        self.mask_framebuffer.unbind();
    }

    /// Binds and clears the apply framebuffer to be drawn onto
    pub unsafe fn begin_draw(&self) {
        self.apply_framebuffer.bind();
        self.apply_framebuffer.clear();
    }

    pub unsafe fn end_draw(&self) {
        self.apply_framebuffer.unbind();
    }

    pub unsafe fn render(&self, window: &mut Window) {
        window.renderer.color_mult_shader.bind();

        window.renderer.color_mult_shader.u_put_int("texture0", vec![0]);
        window.renderer.color_mult_shader.u_put_int("texture1", vec![1]);

        // Bind TEXTURE1 first, because texture0 is re-bound to 0 after
        ActiveTexture(TEXTURE1);
        self.mask_framebuffer.bind_texture();

        ActiveTexture(TEXTURE0);
        self.apply_framebuffer.bind_texture();

        window.renderer.draw_texture_rect(&Bounds::from_ltrb(0.0, window.height as f32, window.width as f32, 0.0),0x00000000);

        window.renderer.color_mult_shader.unbind();
    }
}