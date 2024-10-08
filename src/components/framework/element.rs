use std::ops::Deref;
use std::sync::{Arc, Mutex};
use glfw::ffi::MOUSE_BUTTON_1;
use glfw::{Action, MouseButton};
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::position::Pos;
use crate::components::render::stack::State;

pub trait UIHandler {
    unsafe fn handle(&mut self, event: &Event) -> bool;
    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool;
}

pub struct Element {
    bounds: Bounds,
    last_bounds: Bounds,
    handler: Arc<Mutex<Box<dyn FnMut(&mut Self, &Event)>>>,
    should_render_fn: Arc<Mutex<Box<dyn FnMut(&mut Self) -> bool>>>,
    hovering: bool,
    children: Vec<Box<dyn UIHandler>>,
    pub draggable: bool,
    dragging: (bool, Pos),
}

impl Element {
    pub fn new<B: Into<Bounds>, H: FnMut(&mut Self, &Event) + 'static>(bounds: B, draggable: bool, handler: H, children: Vec<Box<dyn UIHandler>>) -> Element {
        let b = bounds.into();
        Element {
            bounds: b.clone(),
            last_bounds: b,
            handler: Arc::new(Mutex::new(Box::new(handler))),
            should_render_fn: Arc::new(Mutex::new(Box::new((|el| false)))),
            hovering: false,
            children,
            draggable,
            dragging: (false, Pos::new(0.0,0.0))
        }
    }
    pub fn bounds(&mut self) -> &mut Bounds {
        &mut self.bounds
    }
    pub fn hovering(&self) -> bool {
        self.hovering
    }
}

impl UIHandler for Element {
    unsafe fn handle(&mut self, event: &Event) -> bool {
        let mut handled = false;
        let mouse = context().window().mouse();
        self.last_bounds = self.bounds;
        match event {
            Event::PreRender => {
                self.hovering = mouse.pos().intersects(self.bounds());

                if self.dragging.0 {
                    // Set the dragging offset
                    let new = mouse.pos().clone() - self.dragging.1;
                    self.bounds().set_pos(&new);
                    handled = true;
                }
            }
            Event::Render(_) => {
            }
            _ => {}
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
                            if self.hovering && self.draggable {
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

    unsafe fn should_render(&mut self, rp: &RenderPass) -> bool {
        let mut result = self.bounds != self.last_bounds;

        if !result {
            let mut fn_ref = self.should_render_fn.clone();
            result = (fn_ref.lock().unwrap())(self);
        }
        for c in &mut self.children {
            if result {
                break;
            }
            result = c.should_render(rp);
        }

        result
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

    pub fn handler<H: FnMut(&mut Element, &Event) + 'static>(&mut self, handler: H) { self.element.handler = Arc::new(Mutex::new(Box::new(handler))); }
    pub fn should_render<H: FnMut(&mut Element) -> bool + 'static>(&mut self, should_render: H) { self.element.should_render_fn = Arc::new(Mutex::new(Box::new(should_render))); }
    pub fn child<C: UIHandler + 'static>(&mut self, child: C) { self.element.children.push(Box::new(child)); }
    pub fn bounds(&mut self, bounds: Bounds) { self.element.bounds = bounds; }
    pub fn draggable(&mut self, draggable: bool) { self.element.draggable = draggable; }

    pub fn build(self) -> Element {
        self.element
    }
}