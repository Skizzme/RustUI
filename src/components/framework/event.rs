use glfw::{Action, Key, Modifiers, MouseButton, WindowEvent};

#[derive(Debug, PartialEq)]
pub enum Event {
    PreRender,
    Render(RenderPass),
    PostRender,
    MouseClick(MouseButton, Action),
    MousePos(f32, f32),
    Scroll(f32, f32),
    Keyboard(Key, Action, Modifiers),
    Resize(f32, f32),
    GlfwRaw(WindowEvent),
}

impl Event {
    pub fn is_render(&self, pass: RenderPass) -> bool {
        match self {
            Event::Render(e_pass) => &pass == e_pass,
            _ => false
        }
    }
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

    pub fn is_main(&self) -> bool {
        self == &RenderPass::Main
    }
}