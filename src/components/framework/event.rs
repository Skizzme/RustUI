use glfw::{Action, Key, Modifiers, MouseButton};

pub enum Event {
    Render(RenderPass),
    MouseClick(MouseButton, Action),
    Keyboard(Key, Action, Modifiers),
}

pub enum RenderPass {
    Main,
    Blur,
    Bloom,
    Custom(u32),
}