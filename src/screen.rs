use std::time::Duration;
use crate::animation::{FixedAnimation, AnimationType};
use crate::renderer::Renderer;

pub trait GuiScreen {
    fn new(screen_metrics: Screen) -> impl GuiScreen;

    unsafe fn draw(&mut self);

    fn key_press(&self);

    fn metrics(&self) -> &Screen;
}

pub struct Screen {
    pub(crate) screen_width: u32,
    pub(crate) screen_height: u32,
    pub(crate) mouse_x: f32,
    pub(crate) mouse_y: f32,
    pub(crate) frame_delta: f64,
    pub(crate) renderer: Renderer,
}

pub struct MainScreen {
    pub(crate) screen: Screen,
    move_x: FixedAnimation,
    move_y: FixedAnimation,
}

impl GuiScreen for MainScreen {
    fn new(screen_metrics: Screen) -> Self {
        MainScreen {
            screen: screen_metrics,
            move_x: FixedAnimation::new(0.0, 0.0, Duration::from_millis(10000), AnimationType::Linear),
            move_y: FixedAnimation::new(0.0, 0.0, Duration::from_millis(10000), AnimationType::Linear),
        }
    }

    unsafe fn draw(&mut self) {
        let s = &self.screen;
        s.renderer.draw_rect(0.0, 0.0, 2.0, 2.0, 0xffff1213);
        s.renderer.draw_rounded_rect(10.0, 10.0, 100.0, 100.0, 15.0, 0xff909090);
        self.move_x.target = s.mouse_x as f64;
        self.move_y.target = s.mouse_y as f64;
        s.renderer.draw_rounded_rect(self.move_x.get_value() as f32, self.move_y.get_value() as f32, self.move_x.get_value() as f32 + 200.0, self.move_y.get_value() as f32 + 200.0, 25.0, 0xff909090);
    }

    fn key_press(&self) {

    }

    fn metrics(&self) -> &Screen {
        &self.screen
    }
}