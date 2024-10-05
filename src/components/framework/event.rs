use glfw::{Action, Key, Modifiers, MouseButton, WindowEvent};

#[derive(Debug)]
pub enum Event {
    Render(RenderPass),
    MouseClick(MouseButton, Action),
    Keyboard(Key, Action, Modifiers),
    GlfwRaw(WindowEvent),
}

#[derive(Debug)]
pub enum RenderPass {
    Main,
    Blur,
    Bloom,
    Custom(u32),
}