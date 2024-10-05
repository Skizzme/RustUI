use glfw::{Action, MouseButton};
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::framework::element::Element;
use crate::components::framework::event::Event;

pub trait ScreenTrait {
    unsafe fn event(&mut self, event: &Event);
    unsafe fn register_elements(&mut self) -> Vec<Element>;
}

pub struct DefaultScreen;

impl DefaultScreen {
    pub fn new() -> Self {
        DefaultScreen {}
    }
}

impl ScreenTrait for DefaultScreen {
    unsafe fn event(&mut self, event: &Event) {
        match event {
            Event::Render(_) => {}
            Event::MouseClick(_, _) => {}
            Event::Keyboard(_, _, _) => {}
            _ => {}
        }
    }

    unsafe fn register_elements(&mut self) -> Vec<Element> {
        vec![]
    }
}