use glfw::{Action, Key, Modifiers, MouseButton, WindowEvent};
use crate::components::framework::layer::Layer;
use crate::components::framework::layout::LayoutEvent;

pub enum EventResult {
    Error(String),
    LayoutError,
    Ok,
    Used,
}

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
    Layout(LayoutEvent),
}

impl Event {
    pub fn is_render(&self, pass: RenderPass) -> bool {
        match self {
            Event::Render(e_pass) => &pass == e_pass,
            _ => false
        }
    }
    pub fn is_prerender(&self) -> bool {
        match self {
            Event::PreRender => true,
            _ => false,
        }
    }
    pub fn is_mouse_click(&self) -> bool {
        match self {
            Event::MouseClick(_, _) => true,
            _ => false,
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