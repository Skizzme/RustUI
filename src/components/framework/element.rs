use std::sync::{Arc, Mutex};
use crate::components::bounds::Bounds;
use crate::components::framework::event::Event;

pub struct Element {
    bounds: Bounds,
    handler: Arc<Mutex<Box<dyn FnMut(&mut Self, &Event)>>>,
}

impl Element {
    pub fn new<B: Into<Bounds>, H: FnMut(&mut Self, &Event) + 'static>(bounds: B, handler: H) -> Element {
        Element {
            bounds: bounds.into(),
            handler: Arc::new(Mutex::new(Box::new(handler))),
        }
    }
    pub fn bounds(&mut self) -> &mut Bounds {
        &mut self.bounds
    }
    pub fn handle(&mut self, event: &Event) {
        let mut h = (self.handler.clone());
        (h.lock().unwrap())(self, event);
    }
}