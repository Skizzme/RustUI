use std::fs::read;
use std::path;

use crate::components::elements::Drawable;
use crate::components::render::animation::{Animation, AnimationType};
use crate::components::render::bounds::Bounds;
use crate::components::render::font::ScaleMode;
use crate::components::render::mask::FramebufferMask;
use crate::components::window::Window;

pub struct DrawThing {
    pub bounds: Bounds,
    target: f32,
    animator: Animation,
    mask: FramebufferMask,
}

impl DrawThing {
    pub unsafe fn new(bounds: Bounds, w: &mut Window) -> DrawThing {
        w.fonts.set_font_bytes("ProductSans", read("src/assets/fonts/Comfortaa-Light.ttf".replace("/", path::MAIN_SEPARATOR_STR)).unwrap());
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
        if self.target == 0.0 && self.animator.value() <= 0.001 {
            self.target = 1.0
        }
        self.animator.animate_target(self.target as f64, 0.5, AnimationType::Sin, window);
        window.renderer.draw_rect(&Bounds::from_ltrb(20.0, 20.0, 200.0, 100.0), 0xff909090);
        // let (w, h) = window.fonts.get_font("ProductSans", true).scale_mode(ScaleMode::Quality).draw_string(60.0, "Test", 20.0, 20.0, 0xffffffff);

        self.mask.begin_mask();
        let (w, _h) = window.fonts.get_font("ProductSans", true).unwrap().scale_mode(ScaleMode::Quality).draw_string(60.0, "Test", 20.0, 20.0, 0xffffffff);
        // let (w, h) = (200.0, 200.0);
        self.mask.end_mask();

        self.mask.begin_draw();
        let l_color = 0xff00ff00;
        let m_color = 0xff0000ff;
        let r_color = 0xffff0000;
        window.renderer.draw_gradient_rect(&Bounds::from_xywh(20.0, 0.0, w/2.0, 100.0), (l_color, m_color, m_color, l_color));
        window.renderer.draw_gradient_rect(&Bounds::from_xywh(20.0+w/2.0, 0.0, w/3.0, 100.0), (m_color, r_color, r_color, m_color));
        self.mask.end_draw();

        self.mask.render(window);

        // self.bounds.draw_bounds(window, 0xffffffff);
        if self.target == 1.0 && self.animator.value() >= 0.999 {
            self.target = 0.0;
        }
    }

    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}