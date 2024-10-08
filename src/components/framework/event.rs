use glfw::{Action, Key, Modifiers, MouseButton, WindowEvent};

#[derive(Debug)]
pub enum Event {
    PreRender,
    Render(RenderPass),
    PostRender,
    MouseClick(MouseButton, Action),
    Keyboard(Key, Action, Modifiers),
    GlfwRaw(WindowEvent),
}

#[derive(Debug, Hash, PartialEq, Clone)]
pub enum RenderPass {
    Main,
    Blur,
    Post,
    Custom(String),
}

impl Eq for RenderPass {}

impl RenderPass {
    pub fn all() -> Vec<RenderPass> {
        vec![
            RenderPass::Main,
            RenderPass::Blur,
            RenderPass::Post,
        ]
    }
}