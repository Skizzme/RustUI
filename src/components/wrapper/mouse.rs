use std::collections::HashSet;
use glfw::{Action, MouseButton, WindowEvent};
use crate::components::position::Pos;

pub struct Mouse {
    pub(super) pos: Pos,
    pub(super) pressed: HashSet<MouseButton>,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            pos: Pos::new(0.0,0.0),
            pressed: HashSet::new(),
        }
    }

    pub fn handle(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseButton(button, action, mods) => {
                match action {
                    Action::Release => self.pressed.remove(button),
                    Action::Press | Action::Repeat => self.pressed.insert(button.clone()),
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
}