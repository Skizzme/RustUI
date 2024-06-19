use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};

#[allow(unused)]
pub struct MouseEvent {
    scroll: i32,
    clicked_button: i32,
    // TODO
}

#[derive(Clone, Copy)]
pub struct KeyboardEvent {
    key: Key,
    code: Scancode,
    action: Action,
    mods: Modifiers,
}

impl KeyboardEvent {
    pub fn from_windowevent(event: WindowEvent) -> Option<KeyboardEvent> {
        match event {
            WindowEvent::Key(key, code, action, mods)=> {
                Some(KeyboardEvent::new(key, code, action, mods))
            }
            _ => None,
        }
    }

    pub fn new(key: Key, code: Scancode, action: Action, mods: Modifiers) -> KeyboardEvent {
        KeyboardEvent {
            key,
            code,
            action,
            mods
        }
    }
}