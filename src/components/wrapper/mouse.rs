use std::collections::HashSet;

use glfw::{Action, MouseButton, WindowEvent};

use crate::components::position::Vec2;

pub struct Mouse {
    pub(crate) pos: Vec2,
    last_pos: Vec2,
    click_pos: Vec2,
    pub(super) delta: Vec2,
    pub(super) pressed: HashSet<MouseButton>,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            pos: Vec2::new(0.0, 0.0),
            last_pos: Vec2::new(0.0, 0.0),
            click_pos: Vec2::new(0.0, 0.0),
            delta: Vec2::new(0.0, 0.0),
            pressed: HashSet::new(),
        }
    }

    pub fn frame(&mut self) {
        self.delta = self.pos - self.last_pos;
        self.last_pos = self.pos.clone();
    }

    pub fn handle(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseButton(button, action, _) => {
                match action {
                    Action::Release => self.pressed.remove(button),
                    Action::Press | Action::Repeat => {
                        self.click_pos = self.pos;
                        println!("{:?}", self.click_pos());
                        self.pressed.insert(button.clone())
                    },
                };
            }
            WindowEvent::CursorPos(x, y) => {
                self.pos.x = *x as f32;
                self.pos.y = *y as f32;
            }
            _ => {}
        }
    }

    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }
    pub fn is_pressed(&self, button: MouseButton) -> bool {
        self.pressed.contains(&button)
    }
    pub fn delta(&self) -> Vec2 { self.delta }
    pub fn click_pos(&self) -> Vec2 {
        self.click_pos
    }
}