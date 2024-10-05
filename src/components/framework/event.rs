use glfw::{Action, Key, Modifiers, MouseButton, WindowEvent};

pub enum Event {
    Render(RenderPass),
    MouseClick(MouseButton, Action),
    Keyboard(Key, Action, Modifiers),
    GlfwRaw(WindowEvent),
}

pub enum RenderPass {
    Main,
    Blur,
    Bloom,
    Custom(u32),
}