use std::collections::HashSet;
use glfw::Key;

pub struct Keyboard {
    pressed: HashSet<Key>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            pressed: Default::default(),
        }
    }

    pub fn shift(&self) -> bool {
        self.is_pressed(&Key::LeftShift) || self.is_pressed(&Key::RightShift)
    }

    pub fn ctrl(&self) -> bool {
        self.is_pressed(&Key::LeftControl) || self.is_pressed(&Key::RightControl)
    }

    pub fn alt(&self) -> bool {
        self.is_pressed(&Key::LeftAlt) || self.is_pressed(&Key::RightAlt)
    }

    pub fn is_pressed(&self, key: &Key) -> bool {
        self.pressed.contains(key)
    }

    pub fn pressed(&mut self) -> &mut HashSet<Key> {
        &mut self.pressed
    }
}