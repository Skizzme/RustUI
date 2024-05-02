use std::time::{Duration, Instant};
use glfw::{Action, Key, Modifiers, Scancode};
use crate::animation::{AnimationType, Animation};
use crate::renderer::Renderer;
use crate::WindowM;

pub trait GuiScreen {
    fn new() -> impl GuiScreen;

    unsafe fn draw(&mut self, metrics: &WindowM);

    fn key_press(&mut self, key: Key, code: Scancode, action: Action, mods: Modifiers);
}

pub struct MainScreen {
    move_progressive: Animation,
    move_log: Animation,
    move_cubic: Animation,
    target: f64,
}

impl GuiScreen for MainScreen {
    fn new() -> Self {
        MainScreen {
            move_progressive: Animation::new(),
            move_log: Animation::new(),
            move_cubic: Animation::new(),
            target: 200f64,
        }
    }

    unsafe fn draw(&mut self, m: &WindowM) {
        self.move_progressive.animate(m.mouse_x as f64, 0.1f64, AnimationType::Progressive(1f64), m);
        self.move_cubic.animate(m.mouse_x as f64, 1f64, AnimationType::CubicIn, m);
        // m.renderer.draw_rounded_rect(self.move_progressive.get_value() as f32, 10.0, self.move_progressive.get_value() as f32 + 200.0, 10.0 + 100.0, 10.0, 0xff909090);
        // m.renderer.draw_rounded_rect(self.move_cubic.get_value() as f32, 230.0, self.move_cubic.get_value() as f32 + 200.0, 330.0, 10.0, 0xff909090);
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
}