use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

use glfw::{Action, MouseButton};
use parking_lot::Mutex;

use crate::components::context::context;
use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::changing::Changing;
use crate::components::framework::element::ui_traits::{UIHandler, UIIdentifier};
use crate::components::framework::event::{Event, RenderPass};
use crate::components::render::color::ToColor;
use crate::components::render::stack::State;
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;

pub mod ui_traits;
pub mod comp_element;

pub struct Element {
    id: u64,
    bounds: Changing<Vec4>,
    hovering: bool,
    children: Vec<Box<dyn UIHandler>>,

    pub active: bool,
    last_active: bool,
    pub draggable: bool,
    pub scrollable: bool,

    scroll: Changing<(f32, f32)>,
    dragging: (bool, Vec2),
    has_rendered: bool,
    animations: AnimationRegistry,

    handler: Arc<Mutex<Box<dyn FnMut(&mut Self, &Event)>>>,
    render_handler: Option<Arc<Mutex<Box<dyn FnMut(&mut Self, &RenderPass)>>>>,
    should_render_fn: Arc<Mutex<Box<dyn FnMut(&mut Self, &RenderPass) -> bool>>>,
    active_fn: Option<Box<dyn FnMut() -> bool>>,
}

impl Element {
    pub fn new<B, H>(vec4: B, draggable: bool, handler: H, children: Vec<Box<dyn UIHandler>>) -> Element
        where B: Into<Vec4>,
              H: FnMut(&mut Self, &Event) + 'static,
    {
        let b = vec4.into();
        Element {
            id: ui_traits::random_id(),
            bounds: Changing::new(b.clone()),
            handler: Arc::new(Mutex::new(Box::new(handler))),
            render_handler: None,
            should_render_fn: Arc::new(Mutex::new(Box::new(|_, _| false))),
            hovering: false,
            children,
            active: true,
            last_active: true,
            draggable,
            scrollable: false,
            scroll: Changing::new((0.0, 0.0)),
            dragging: (false, Vec2::new(0.0, 0.0)),
            has_rendered: false,
            animations: AnimationRegistry::new(),
            active_fn: None,
        }
    }
    // pub fn text(mut fr: FontRenderer, size: f32, text: impl ToString, pos: impl Into<Vec2>, color: impl ToColor) -> Element {
    //     let pos = pos.into();
    //     let text = text.to_string();
    //     let color = color.to_color();
    //
    //     let text_c = text.clone();
    //     let builder = ElementBuilder::new()
    //         .bounds(Vec4::xywh(pos.x, pos.y, 0.0, 0.0))
    //         .handler(move |el, event| unsafe {
    //             match event {
    //                 Event::Render(pass) => {
    //                     match pass {
    //                         RenderPass::Main => {
    //                             let (_, b) = fr.draw_string((size, &text_c, color), el.bounds.current_mut().top_left());
    //                             el.bounds().set_width(b.width());
    //                             el.bounds().set_height(b.height());
    //                         }
    //                         _ => {}
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //         })
    //         .draggable(true)
    //         .should_render(|_, _| true);
    //
    //     builder.build()
    // }
    pub fn set_bounds(&mut self, vec4: Vec4) {
        self.bounds.set(vec4);
    }
    pub fn bounds(&mut self) -> &mut Vec4 {
        self.bounds.current_mut()
    }
    pub fn hovering(&self) -> bool {
        self.hovering
    }
    pub fn scroll(&mut self) -> &mut Changing<(f32, f32)> {
        &mut self.scroll
    }
    pub fn set_active_fn<Fn: FnMut() -> bool + 'static>(&mut self, active_fn: Option<Fn>) {
        match active_fn {
            None => self.active_fn = None,
            Some(active_fn) => {
                self.active_fn = Some(Box::new(active_fn));
            }
        }
    }
    fn dispatch_event(&mut self, event: &Event) {
        // Arc mutex so that can be called with self ref
        let h = self.handler.clone();
        (h.lock())(self, event);
        match event {
            Event::Render(pass) => {
                match self.render_handler.clone() {
                    None => {}
                    Some(r) => {
                        (r.lock())(self, pass)
                    }
                }
            }
            _ => {}
        }
    }
}

impl UIHandler for Element {
    unsafe fn handle(&mut self, event: &Event) -> bool {
        let mut handled = false;
        let mouse = context().window().mouse();
        match event {
            Event::PreRender => {
                if let Some(active_fn) = &mut self.active_fn {
                    self.active = active_fn();
                }
                self.hovering = mouse.pos().intersects(self.bounds());
                self.dispatch_event(event);
            }
            Event::PostRender => {
                self.last_active = self.active;
            }
            _ => {}
        }

        if !self.active {
            return false;
        }

        match event {
            Event::PreRender => {

                if self.dragging.0 {
                    // Set the dragging offset
                    let new = mouse.pos().clone() - self.dragging.1;
                    self.bounds().set_pos(new);
                    handled = true;
                }
            }
            Event::Render(pass) => { match pass {
                RenderPass::Main => {
                    // self.bounds().debug_draw(0xffffffff)
                },
                _ => {}
            }}
            Event::PostRender => {
                self.has_rendered = true;
                self.scroll().update();
                self.bounds.update();
            }
            _ => {}
        }

        match event {
            Event::PreRender => {} // dont dispatch pre render twice
            _ => {
                self.dispatch_event(event);
            }
        }

        // Translate child positions, which also offsets mouse correctly
        context().renderer().stack().push(State::Translate(self.bounds().x(), self.bounds().y()));
        // println!("transled to {:?}", self.bounds);
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
            // TODO maybe this should only return once all children are handled?
            if c.handle(event) {
                context().renderer().stack().pop();
                return true;
            }
        }
        context().renderer().stack().pop();
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
                                self.dragging = (true, mouse.pos().clone() - self.bounds().pos());
                                true
                            } else { false }
                        },
                        _ => false,
                    }
                } else { false }
            },
            Event::Scroll(x, y) => {
                if self.hovering {
                    let mut updated = *self.scroll.current_mut();
                    updated.0 += *x;
                    updated.1 += *y;
                    self.scroll().set(updated);

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
        let changed_active = self.last_active != self.active;
        if !self.active && !changed_active {
            return false;
        }
        if !self.has_rendered || self.scroll.changed() || self.bounds.changed() || changed_active {
            return true;
        }

        let fn_ref = self.should_render_fn.clone();
        // println!("check call");
        if (fn_ref.lock())(self, rp) {
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

        false
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        Some(&mut self.animations)
    }
}

impl UIIdentifier for Element {
    fn ui_id(&self) -> u64 {
        self.id
    }
}

pub struct ElementBuilder {
    element: Element,
}

impl ElementBuilder {
    pub fn new() -> Self {
        ElementBuilder {
            element: Element::new(Vec4::ltrb(0.0, 0.0, 0.0, 0.0), false, |_, _| {}, vec![])
        }
    }

    pub fn handler<H: FnMut(&mut Element, &Event) + 'static>(mut self, handler: H) -> Self {
        self.element.handler = Arc::new(Mutex::new(Box::new(handler))); self
    }
    pub fn render_handler<R: FnMut(&mut Element, &RenderPass) + 'static>(mut self, render_handler: R) -> Self {
        self.element.render_handler = Some(Arc::new(Mutex::new(Box::new(render_handler)))); self
    }
    pub fn should_render<H: FnMut(&mut Element, &RenderPass) -> bool + 'static>(mut self, should_render: H) -> Self {
        self.element.should_render_fn = Arc::new(Mutex::new(Box::new(should_render))); self
    }
    pub fn child<C: UIHandler + 'static>(mut self, child: C) -> Self {
        self.element.children.push(Box::new(child)); self
    }
    pub fn bounds(mut self, vec4: Vec4) -> Self {
        self.element.bounds.set(vec4); self
    }
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.element.draggable = draggable; self
    }
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.element.scrollable = scrollable; self
    }
    pub fn active(mut self, active: bool) -> Self {
        self.element.active = active; self
    }
    pub fn active_fn<Fn: FnMut() -> bool + 'static>(mut self, active_fn: Option<Fn>) -> Self {
        self.element.set_active_fn(active_fn); self
    }
    pub fn animations(&mut self) -> &mut AnimationRegistry {
        &mut self.element.animations
    }
    pub fn build(self) -> Element {
        self.element
    }
}