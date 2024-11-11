use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use glfw::ffi::MOUSE_BUTTON_1;
use glfw::{Action, MouseButton};
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::framework::animation::{Animation, AnimationRef, AnimationRegistry, AnimationRegTrait};
use crate::components::framework::event::{Event, RenderPass};
use crate::components::position::Vec2;
use crate::components::render::color::{Color, ToColor};
use crate::components::render::font::renderer::FontRenderer;
use crate::components::render::stack::State;

pub trait UIHandler {
    unsafe fn handle(&mut self, event: &Event) -> bool;
    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool;
    fn animations(&mut self) -> Option<&mut AnimationRegistry>;
}

pub struct Element {
    bounds: Bounds,
    last_bounds: Bounds,
    handler: Arc<Mutex<Box<dyn FnMut(&mut Self, &Event)>>>,
    should_render_fn: Arc<Mutex<Box<dyn FnMut(&mut Self, &RenderPass) -> bool>>>,
    hovering: bool,
    dyn_child: Arc<Mutex<Box<dyn FnMut(&mut Self) -> Option<Box<dyn Iterator<Item=Box<dyn UIHandler>>>>>>>,
    children: Vec<Box<dyn UIHandler>>,
    pub draggable: bool,
    pub scrollable: bool,
    scroll: (f32, f32),
    last_scroll: (f32, f32),
    dragging: (bool, Vec2),
    has_rendered: bool,
    animations: AnimationRegistry,
}

impl Element {
    pub fn new<B, H, C>(bounds: B, draggable: bool, handler: H, children: Vec<Box<dyn UIHandler>>, dyn_children: C) -> Element
    where B: Into<Bounds>,
          H: FnMut(&mut Self, &Event) + 'static,
          C: FnMut(&mut Self) -> Option<Box<dyn Iterator<Item=Box<dyn UIHandler>>>> + 'static
    {
        let b = bounds.into();
        Element {
            bounds: b.clone(),
            last_bounds: b,
            handler: Arc::new(Mutex::new(Box::new(handler))),
            should_render_fn: Arc::new(Mutex::new(Box::new((|_, _| false)))),
            hovering: false,
            dyn_child: Arc::new(Mutex::new(Box::new(dyn_children))),
            children,
            draggable,
            scrollable: false,
            scroll: (0.0, 0.0),
            last_scroll: (0.0, 0.0),
            dragging: (false, Vec2::new(0.0, 0.0)),
            has_rendered: false,
            animations: AnimationRegistry::new(),
        }
    }
    pub fn text(mut fr: FontRenderer, size: f32, text: impl ToString, pos: impl Into<Vec2>, color: impl ToColor) -> Element {
        let pos = pos.into();
        let text = text.to_string();
        let color = color.to_color();

        let pos_c = pos.clone();
        let text_c = text.clone();
        let mut builder = ElementBuilder::new()
            .bounds(Bounds::xywh(pos.x, pos.y, 0.0, 0.0))
            .handler(move |el, event| unsafe {
                match event {
                    Event::Render(pass) => {
                        match pass {
                            RenderPass::Main => {
                                let (width, height) = fr.draw_string_inst((size, &text_c, color), el.bounds.top_left());
                                el.bounds().set_width(width);
                                el.bounds().set_height(height);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            })
            .draggable(true)
            .should_render(|_, _| true);

        builder.build()
    }
    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = bounds;
    }
    pub fn bounds(&mut self) -> &mut Bounds {
        &mut self.bounds
    }
    pub fn hovering(&self) -> bool {
        self.hovering
    }
    pub fn scroll(&self) -> (f32, f32) {
        self.scroll
    }
}

impl UIHandler for Element {
    unsafe fn handle(&mut self, event: &Event) -> bool {
        let mut handled = false;
        let mouse = context().window().mouse();
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
            Event::Render(pass) => {
                // match pass {
                //     RenderPass::Main => self.bounds().draw_bounds(0xffffffff),
                //     _ => {}
                // }
            }
            Event::PostRender => {
                self.has_rendered = true;
                self.last_bounds = self.bounds.clone();
                self.last_scroll = self.scroll();
            }
            _ => {}
        }

        // Arc mutex so that can be called with self ref
        let mut h = (self.handler.clone());
        (h.lock().unwrap())(self, event);

        // Translate child positions, which also offsets mouse correctly
        context().renderer().stack().push(State::Translate(self.bounds.x(), self.bounds.y()));
        for c in &mut self.children {
            match event {
                Event::PostRender => {
                    match c.animations() {
                        None => {}
                        Some(reg) => { reg.post(); }
                    }
                }
                _ => {}
            }
            if c.handle(event) {
                context().renderer().stack().pop();
                return true;
            }
        }
        let popped = context().renderer().stack().pop();
        // println!("popped {:?}", popped);

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
                        Action::Press => {
                            if self.hovering && self.draggable {
                                self.dragging = (true, mouse.pos().clone() - self.bounds.pos());
                                true
                            } else { false }
                        },
                        _ => false,
                    }
                } else { false }
            },
            Event::Scroll(x, y) => {
                if self.hovering {
                    self.scroll.0 += *x;
                    self.scroll.1 += *y;
                    println!("{:?}", self.scroll);
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        handled
    }


    unsafe fn should_render(&mut self, rp: &RenderPass) -> bool {
        if !self.has_rendered || self.bounds != self.last_bounds || self.last_scroll != self.scroll  {
            return true;
        }

        let mut fn_ref = self.should_render_fn.clone();
        // println!("check call");
        if (fn_ref.lock().unwrap())(self, rp) {
            return true;
        }

        for a in self.animations().unwrap().all() {
            if a.borrow().has_changed() {
                return true;
            }
        }

        for c in &mut self.children {
            if c.should_render(rp) {
                return true
            }
        }

        let cl = self.dyn_child.clone();
        match (cl.lock().unwrap())(self) {
            None => {}
            Some(mut children) => {
                for mut c in &mut children {
                    if c.should_render(rp) {
                        return true
                    }
                }
            }
        }

        false
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        Some(&mut self.animations)
    }
}

pub struct ElementBuilder {
    element: Element,
}

impl ElementBuilder {
    pub fn new() -> Self {
        ElementBuilder {
            element: Element::new(Bounds::ltrb(0.0,0.0,0.0,0.0), false, |_, _| {}, vec![], |_| None)
        }
    }

    pub fn handler<H: FnMut(&mut Element, &Event) + 'static>(mut self, handler: H) -> Self { self.element.handler = Arc::new(Mutex::new(Box::new(handler))); self }
    pub fn should_render<H: FnMut(&mut Element, &RenderPass) -> bool + 'static>(mut self, should_render: H) -> Self { self.element.should_render_fn = Arc::new(Mutex::new(Box::new(should_render))); self }
    pub fn child<C: UIHandler + 'static>(mut self, child: C) -> Self { self.element.children.push(Box::new(child)); self }
    pub fn bounds(mut self, bounds: Bounds) -> Self { self.element.bounds = bounds; self }
    pub fn draggable(mut self, draggable: bool) -> Self { self.element.draggable = draggable; self }
    pub fn scrollable(mut self, scrollable: bool) -> Self { self.element.scrollable = scrollable; self }
    pub fn animations(&mut self) -> &mut AnimationRegistry { &mut self.element.animations }
    pub fn children<C: FnMut(&mut Element) -> Option<Box<dyn Iterator<Item=Box<dyn UIHandler>>>> + 'static>(mut self, children: C) -> Self { self.element.dyn_child = Arc::new(Mutex::new(Box::new(children))); self }
    pub fn build(self) -> Element {
        self.element
    }
}