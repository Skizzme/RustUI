use std::collections::HashSet;

use glfw::{Action, MouseButton, WindowEvent};

use crate::components::position::Pos;

pub struct Mouse {
    pub(super) pos: Pos,
    last_pos: Pos,
    click_pos: Pos,
    pub(super) delta: Pos,
    pub(super) pressed: HashSet<MouseButton>,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            pos: Pos::new(0.0,0.0),
            last_pos: Pos::new(0.0,0.0),
            click_pos: Pos::new(0.0,0.0),
            delta: Pos::new(0.0, 0.0),
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

    pub fn pos(&self) -> &Pos {
        &self.pos
    }
    pub fn is_pressed(&self, button: MouseButton) -> bool {
        self.pressed.contains(&button)
    }
    pub fn delta(&self) -> Pos { self.delta }
    pub fn click_pos(&self) -> Pos {
        self.click_pos
    }
}