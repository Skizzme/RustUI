use glfw::{Action, Key, Modifiers, MouseButton, WindowEvent};

#[derive(Debug)]
pub enum Event {
    PreRender,
    Render(RenderPass),
    MouseClick(MouseButton, Action),
    Keyboard(Key, Action, Modifiers),
    GlfwRaw(WindowEvent),
}

#[derive(Debug, Hash, PartialEq, Clone)]
pub enum RenderPass {
    Main,
    Post,
    Custom(String),
}

impl Eq for RenderPass {
}