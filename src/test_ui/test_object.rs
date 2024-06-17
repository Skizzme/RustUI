use std::ops::Sub;
use crate::components::elements::Drawable;
use crate::components::render::animation::{Animation, AnimationType};
use crate::components::window::Window;
use crate::components::render::bounds::Bounds;
use crate::gl_binds::gl30;

pub struct DrawThing {
    pub bounds: Bounds,
    target: f32,
    animator: Animation
}

impl DrawThing {
    pub fn new(bounds: Bounds) -> DrawThing {
        DrawThing {
            bounds,
            target: 200.0,
            animator: Animation::new(),
        }
    }
}

impl Drawable for DrawThing {
    unsafe fn draw<'a>(&mut self, window: &mut Window) {
        if self.target == 0.0 && self.animator.get_value() <= 1.0 {
            self.target = 200.0
        }
        self.animator.animate(self.target as f64, 0.5, AnimationType::Sin, window);
        window.renderer.draw_rect(&self.bounds, 0xff909090);
        self.bounds.set_width(self.animator.get_value() as f32);
        self.bounds.draw_bounds(window, 0xffffffff);
        println!("{:?}", self.bounds);
        if self.target == 200.0 && self.animator.get_value() >= 199.0 {
            self.target = 0.0;
        }
    }

    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}