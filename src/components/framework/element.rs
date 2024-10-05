use std::ops::Deref;
use std::sync::{Arc, Mutex};
use glfw::ffi::MOUSE_BUTTON_1;
use glfw::{Action, MouseButton};
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::framework::event::Event;
use crate::components::position::Pos;
use crate::components::render::stack::State;

pub struct Element {
    bounds: Bounds,
    handler: Arc<Mutex<Box<dyn FnMut(&mut Self, &Event)>>>,
    hovering: bool,
    children: Vec<Element>,
    pub draggable: bool,
    dragging: (bool, Pos),
}

impl Element {
    pub fn new<B: Into<Bounds>, H: FnMut(&mut Self, &Event) + 'static>(bounds: B, draggable: bool, handler: H, children: Vec<Element>) -> Element {
        Element {
            bounds: bounds.into(),
            handler: Arc::new(Mutex::new(Box::new(handler))),
            hovering: false,
            children,
            draggable,
            dragging: (false, Pos::new(0.0,0.0))
        }
    }
    pub fn bounds(&mut self) -> &mut Bounds {
        &mut self.bounds
    }

    pub unsafe fn handle(&mut self, event: &Event) -> bool {
        let mouse = context().window().mouse();
        let mut handled = false;
        self.hovering = mouse.pos().intersects(self.bounds());

        if self.dragging.0 {
            // Set the dragging offset
            let new = mouse.pos().clone() - self.dragging.1;
            self.bounds().set_pos(&new);
            handled = true;
        }

        // Arc mutex so that can be called with self ref
        let mut h = (self.handler.clone());
        (h.lock().unwrap())(self, event);

        // Translate child positions, which also offsets mouse correctly
        context().renderer().stack().push(State::Translate(self.bounds.x(), self.bounds.y()));
        for c in &mut self.children {
            if c.handle(event) {
                context().renderer().stack().pop();
                return true;
            }
        }
        context().renderer().stack().pop();

        // After letting child elements take priority, check if interaction on this object occurred
        handled = handled | match event {
            Event::MouseClick(button, action) => {
                if *button == MouseButton::Button1 {
                    match action {
                        Action::Release =>
                            if self.dragging.0 {
                                self.dragging.0 = false;
                                true
                            } else { false },
                        Action::Press =>
                            if self.hovering {
                                self.dragging = (true, mouse.pos().clone() - self.bounds.pos());
                                true
                            } else { false },
                        _ => false,
                    }
                } else { false }
            },
            _ => false,
        };
        handled
    }
    pub fn hovering(&self) -> bool {
        self.hovering
    }
}

pub struct ElementBuilder {
    element: Element,
}

impl ElementBuilder {
    pub fn new() -> Self {
        ElementBuilder {
            element: Element::new(Bounds::ltrb(0.0,0.0,0.0,0.0), false, |_, _| {}, vec![])
        }
    }

    pub fn handler<H: FnMut(&mut Self, &Event) + 'static>(&mut self, handler: H) { self.element.handler = Arc::new(Mutex::new(Box::new(handler))); }
    pub fn child(&mut self, child: Element) { self.element.children.push(child); }
    pub fn bounds(&mut self, bounds: Bounds) { self.element.bounds = bounds; }
    pub fn draggable(&mut self, draggable: bool) { self.element.draggable = draggable; }

    pub fn build(self) -> Element {
        self.element
    }
}