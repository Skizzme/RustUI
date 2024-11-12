use crate::components::framework::event::{Event, RenderPass};
use crate::components::framework::layer::Layer;

pub trait ScreenTrait {
    unsafe fn handle(&mut self, event: &Event);
    unsafe fn init(&mut self) -> Vec<Layer>; // TODO change this to maybe use some sort of screen properties?
    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool;
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

    unsafe fn init(&mut self) -> Vec<Layer> {
        vec![]
    }

    unsafe fn should_render(&mut self, _: &RenderPass) -> bool {
        true
    }
}