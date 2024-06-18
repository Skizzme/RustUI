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
            target: 0.0,
            animator: Animation::new(),
            mask: FramebufferMask::new(w),
        }
    }
}

impl Drawable for DrawThing {
    unsafe fn draw<'a>(&mut self, window: &mut Window) {
        if self.target == 0.0 && self.animator.get_value() <= 0.001 {
            self.target = 1.0
        }
        self.animator.animate(self.target as f64, 0.5, AnimationType::Sin, window);
        window.renderer.draw_rect(&Bounds::from_ltrb(20.0, 20.0, 200.0, 100.0), 0xff909090);
        // let (w, h) = window.fonts.get_font("ProductSans", true).scale_mode(ScaleMode::Quality).draw_string(60.0, "Test", 20.0, 20.0, 0xffffffff);

        self.mask.begin_mask();
        let (w, h) = window.fonts.get_font("ProductSans", true).scale_mode(ScaleMode::Quality).draw_string(60.0, "Test", 20.0, 20.0, 0xffffffff);
        self.mask.end_mask();

        self.mask.begin_draw();
        let l_color = 0xff00ff00;
        let m_color = 0xff0000ff;
        let r_color = 0xffff0000;
        window.renderer.draw_gradient_rect(&Bounds::from_xywh(20.0, 0.0, w/2.0, 100.0), (l_color, m_color, m_color, l_color));
        window.renderer.draw_gradient_rect(&Bounds::from_xywh(20.0+w/2.0, 0.0, w/2.0, 100.0), (m_color, r_color, r_color, m_color));
        self.mask.end_draw();

        self.mask.render(window);

        // self.bounds.draw_bounds(window, 0xffffffff);
        if self.target == 1.0 && self.animator.get_value() >= 0.999 {
            self.target = 0.0;
        }
    }

    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}