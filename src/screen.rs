use std::time::{Duration, Instant};
use glfw::{Action, Key, Modifiers, Scancode};
use crate::animation::{AnimationType, Animation};
use crate::renderer::Renderer;

pub trait GuiScreen {
    fn new(screen_metrics: Screen) -> impl GuiScreen;

    unsafe fn draw(&mut self);

    fn key_press(&mut self, key: Key, code: Scancode, action: Action, mods: Modifiers);

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
    move_progressive: Animation,
    move_log: Animation,
    move_cubic: Animation,
    target: f64,
}

impl GuiScreen for MainScreen {
    fn new(screen_metrics: Screen) -> Self {
        MainScreen {
            screen: screen_metrics,
            move_progressive: Animation::new(),
            move_log: Animation::new(),
            move_cubic: Animation::new(),
            target: 200f64,
        }
    }

    unsafe fn draw(&mut self) {
        let s = &self.screen;
        s.renderer.draw_rect(0.0, 0.0, 2.0, 2.0, 0xffff1213);
        s.renderer.draw_rounded_rect(10.0, 10.0, 100.0, 100.0, 15.0, 0xff909090);
        // self.move_x.animate(s.mouse_x as f64 - 100.0, 1f64, AnimationType::CubicOut, s);
        // self.move_y.animate(s.mouse_y as f64 - 100.0, 1f64, AnimationType::CubicOut, s);
        self.move_progressive.animate(s.mouse_x as f64, 0.1f64, AnimationType::Progressive(1f64), s);
        self.move_log.animate(s.mouse_x as f64, 0.1f64, AnimationType::QuarticIn, s);
        self.move_cubic.animate(s.mouse_x as f64, 1f64, AnimationType::CubicIn, s);
        s.renderer.draw_rounded_rect(self.move_progressive.get_value() as f32, 10.0, self.move_progressive.get_value() as f32 + 200.0, 10.0 + 100.0, 10.0, 0xff909090);
        s.renderer.draw_rounded_rect(self.move_log.get_value() as f32, 120.0, self.move_log.get_value() as f32 + 200.0, 220.0, 10.0, 0xff909090);
        s.renderer.draw_rounded_rect(self.move_cubic.get_value() as f32, 230.0, self.move_cubic.get_value() as f32 + 200.0, 330.0, 10.0, 0xff909090);
    }

    fn key_press(&mut self, key: Key, code: Scancode, action: Action, mods: Modifiers) {
        match action {
            Action::Release => {}
            Action::Press => {
                println!("press");
                if self.target == 200f64 {
                    self.target = 400f64;
                } else {
                    self.target = 200f64;
                }}
            Action::Repeat => {}
        }
    }

    fn metrics(&self) -> &Screen {
        &self.screen
    }
}