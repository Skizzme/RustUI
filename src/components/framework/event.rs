use glfw::{Action, Key, Modifiers, MouseButton, WindowEvent};

#[derive(Debug)]
pub enum Event {
    PreRender,
    Render(RenderPass),
    PostRender,
    MouseClick(MouseButton, Action),
    Scroll(f32, f32),
    Keyboard(Key, Action, Modifiers),
    Resize(f32, f32),
    GlfwRaw(WindowEvent),
}

#[derive(Debug, Hash, PartialEq, Clone)]
pub enum RenderPass {
    Main,
    Bloom,
    Post,
    Custom(String),
}

impl Eq for RenderPass {}

impl RenderPass {
    pub fn all() -> Vec<RenderPass> {
        vec![
            RenderPass::Main,
            RenderPass::Bloom,
            RenderPass::Post,
        ]
    }
}