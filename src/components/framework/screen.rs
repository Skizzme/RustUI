use glfw::{Action, MouseButton};
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::framework::element::{Element, UIHandler};
use crate::components::framework::event::Event;

pub trait ScreenTrait {
    unsafe fn handle(&mut self, event: &Event);
    unsafe fn init(&mut self) -> Vec<Box<dyn UIHandler>>;
    unsafe fn should_render(&mut self) -> bool;
}

pub struct DefaultScreen;

impl DefaultScreen {
    pub fn new() -> Self {
        DefaultScreen {}
    }
}

impl ScreenTrait for DefaultScreen {
    unsafe fn handle(&mut self, event: &Event) {
        match event {
            Event::Render(_) => {}
            Event::MouseClick(_, _) => {}
            Event::Keyboard(_, _, _) => {}
            _ => {}
        }
    }

    unsafe fn init(&mut self) -> Vec<Box<dyn UIHandler>> {
        vec![]
    }

    unsafe fn should_render(&mut self) -> bool {
        true
    }
}