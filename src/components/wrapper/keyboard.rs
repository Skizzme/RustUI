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

    pub fn is_pressed(&self, key: &Key) -> bool {
        self.pressed.contains(key)
    }

    pub fn pressed(&mut self) -> &mut HashSet<Key> {
        &mut self.pressed
    }
}