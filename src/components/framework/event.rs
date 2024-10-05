use glfw::{Action, Key, Modifiers, MouseButton};

pub enum Event {
    Render(f32),
    MouseClick(MouseButton, Action),
    Keyboard(Key, Action, Modifiers),
}