use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::render::stack::State::Blend;
use crate::components::wrapper::framebuffer::Framebuffer;
use crate::components::wrapper::shader::Shader;
use crate::gl_binds::gl11::RGBA;
use crate::gl_binds::gl20::{ActiveTexture, TEXTURE0, TEXTURE1, TEXTURE2, TEXTURE3};
use crate::gl_binds::gl30::{BLEND, Disable, Enable};

/// Creates a mask based off of brightness in the mask framebuffer, applied onto the draw framebuffer
///
/// If the mask is fully white, then the draw framebuffer will show. If it's black it wont
///
/// Make sure ALL drawing within the mask and apply layer DO NOT HAVE BLEND ENABLED
pub struct FramebufferMask {
    mask_framebuffer: Framebuffer,
    apply_framebuffer: Framebuffer,
}

impl FramebufferMask {
    pub unsafe fn new() -> FramebufferMask {
        let window = context().window();
        FramebufferMask {
            mask_framebuffer: Framebuffer::new(RGBA, window.width, window.height).expect("Failed to make mask framebuffer"),
            apply_framebuffer: Framebuffer::new(RGBA, window.width, window.height).expect("Failed to make apply framebuffer"),
        }
    }

    /// Binds and clears the mask framebuffer to be drawn onto
    pub unsafe fn begin_mask(&mut self) {
        self.mask_framebuffer.bind();
        Framebuffer::clear_current();
        Disable(BLEND);
    }

    pub unsafe fn end_mask(&self) {
        self.mask_framebuffer.unbind();
        Enable(BLEND);
    }

    /// Binds and clears the apply framebuffer to be drawn onto
    pub unsafe fn begin_draw(&mut self) {
        self.apply_framebuffer.bind();
        Framebuffer::clear_current();
        context().renderer().stack().push_l(Blend(false), 1);
    }

    pub unsafe fn end_draw(&self) {
        self.apply_framebuffer.unbind();
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
        self.mask_framebuffer.bind_texture();

        // Bind TEXTURE0 last so that it doesn't have to be set later
        ActiveTexture(TEXTURE1);
        self.apply_framebuffer.bind_texture();

        ActiveTexture(TEXTURE0);

        renderer.draw_texture_rect(&Bounds::ltrb(0.0, window.height as f32, window.width as f32, 0.0), 0x00000000);

        Shader::unbind();
        // Enable(BLEND);
    }
}