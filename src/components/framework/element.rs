use std::ops::Deref;
use std::sync::{Arc, Mutex};
use glfw::MouseButton;
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::framework::event::Event;

pub struct Element {
    bounds: Bounds,
    handler: Arc<Mutex<Box<dyn FnMut(&mut Self, &Event)>>>,
    hovering: bool,
    pub draggable: bool,
}

impl Element {
    pub fn new<B: Into<Bounds>, H: FnMut(&mut Self, &Event) + 'static>(bounds: B, draggable: bool, handler: H) -> Element {
        Element {
            bounds: bounds.into(),
            handler: Arc::new(Mutex::new(Box::new(handler))),
            hovering: false,
            draggable,
        }
    }
    pub fn bounds(&mut self) -> &mut Bounds {
        &mut self.bounds
    }

    pub unsafe fn handle(&mut self, event: &Event) {
        let mouse = context().window().mouse();
        self.hovering = mouse.pos().intersects(self.bounds());
        if self.hovering && self.draggable && mouse.is_pressed(MouseButton::Button1) {
            self.bounds.set_x(mouse.pos().x - mouse.click_pos().x);
            self.bounds.set_y(mouse.pos().y - mouse.click_pos().y);
        }

        let mut h = (self.handler.clone());
        (h.lock().unwrap())(self, event);
    }
    pub fn hovering(&self) -> bool {
        self.hovering
    }
}