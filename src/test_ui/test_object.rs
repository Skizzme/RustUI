use std::fs::read_to_string;
use std::ops::{Mul, Sub};
use gl::{BindTexture, BlendFunc, ClearColor, TEXTURE_2D};
use crate::components::elements::Drawable;
use crate::components::render::animation::{Animation, AnimationType};
use crate::components::window::Window;
use crate::components::render::bounds::Bounds;
use crate::components::render::font::ScaleMode;
use crate::components::render::framebuffer::Framebuffer;
use crate::components::render::mask::FramebufferMask;
use crate::components::render::shader::Shader;
use crate::gl_binds::gl30;
use crate::gl_binds::gl30::{ActiveTexture, ALPHA, BLEND, Enable, ONE_MINUS_SRC_ALPHA, RGBA, SRC_ALPHA, TEXTURE0, TEXTURE1, TEXTURE10};

pub struct DrawThing {
    pub bounds: Bounds,
    target: f32,
    animator: Animation,
    mask: FramebufferMask,
}

impl DrawThing {
    pub unsafe fn new(bounds: Bounds, w: &mut Window) -> DrawThing {
        DrawThing {
            bounds,
            target: 200.0,
            animator: Animation::new(),
            mask: FramebufferMask::new(w),
        }
    }
}

impl Drawable for DrawThing {
    unsafe fn draw<'a>(&mut self, window: &mut Window) {
        if self.target == 0.0 && self.animator.get_value() <= 1.0 {
            self.target = 200.0
        }
        self.animator.animate(self.target as f64, 0.5, AnimationType::Linear, window);
        window.renderer.draw_rect(&Bounds::from_ltrb(00.0, 0.0, window.width as f32, window.height as f32), 0xff131619);
        // window.fonts.get_font("JetBrainsMono-Medium", false).scale_mode(ScaleMode::Quality).draw_string(30.0, "Test", 20.0, 20.0, 0xff00ff00);

        self.mask.begin_mask();
        window.renderer.draw_rect(&self.bounds, 0xff00ffff);
        self.mask.end_mask();
        self.mask.begin_draw();
        window.fonts.get_font("JetBrainsMono-Medium", false).scale_mode(ScaleMode::Quality).draw_string(30.0, "Test", 20.0, 20.0, 0xff00ff00);
        self.mask.end_draw();
        self.mask.render(window);

        self.bounds.set_left(-self.animator.get_value() as f32);
        // self.bounds.draw_bounds(window, 0xffffffff);
        if self.target == 200.0 && self.animator.get_value() >= 199.0 {
            self.target = 0.0;
        }
    }

    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}