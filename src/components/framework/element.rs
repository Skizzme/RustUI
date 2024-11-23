use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use glfw::{Action, MouseButton};
use parking_lot::Mutex;

use crate::components::spatial::vec4::Vec4;
use crate::components::context::context;
use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::changing::Changing;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::spatial::vec2::Vec2;
use crate::components::render::color::ToColor;
use crate::components::render::font::renderer::FontRenderer;
use crate::components::render::stack::State;

pub trait UIHandler {
    unsafe fn handle(&mut self, event: &Event) -> bool;
    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool;
    fn animations(&mut self) -> Option<&mut AnimationRegistry>;
}

pub trait UIIdentifier {
    fn ui_id(&self) -> u64;
}

pub struct MultiElement<IterFn, State, Item, Cons>
    where IterFn: FnMut(Box<dyn for<'a> FnMut(&mut State, &'a mut Item)>),
          Cons: FnMut(bool, &mut State, &mut Item) -> Option<Box<dyn UIHandler>>,
          State: Debug,
          Item: UIIdentifier + Debug,
{
    elements: HashMap<u64, Box<dyn UIHandler>>,
    iter_fn: IterFn,
    item_construct: Arc<Mutex<Cons>>,
    _holder: PhantomData<(State, Item)>,
}

impl<IterFn, State, Item, Cons> MultiElement<IterFn, State, Item, Cons>
    where IterFn: FnMut(Box<dyn for<'a> FnMut(&mut State, &'a mut Item)>),
          Cons: FnMut(bool, &mut State, &mut Item) -> Option<Box<dyn UIHandler>> + 'static,
          State: Debug,
          Item: UIIdentifier + Debug,
{
    pub fn new(iter_fn: IterFn, item_construct: Cons) -> Self {
        MultiElement {
            elements: HashMap::new(),
            iter_fn,
            item_construct: Arc::new(Mutex::new(item_construct)),
            _holder: PhantomData::default(),
        }
    }

    pub fn update_elements(&mut self) {
        let mut new_elements = Rc::new(RefCell::new(HashMap::new()));
        let mut elements = Rc::new(RefCell::new(std::mem::take(&mut self.elements)));

        let cons = self.item_construct.clone();

        let c_elements = elements.clone();
        let c_new_elements = new_elements.clone();
        (self.iter_fn)(Box::new(move |state, item| {
            let id = item.ui_id();
            let exists = c_elements.borrow().contains_key(&id) || c_new_elements.borrow().contains_key(&id);
            let (id, el) = match (cons.lock())(exists, state, item) {
                None => {
                    let id = item.ui_id();
                    let old = c_elements.borrow_mut().remove(&id);
                    (id, old)
                },
                Some(el) => {
                    (item.ui_id(), Some(el))
                },
            };

            match el {
                None => {}
                Some(new) => {
                    c_new_elements.borrow_mut().insert(id, new);
                }
            }
        }));

        let new_elements = Rc::into_inner(new_elements).unwrap().into_inner();
        std::mem::replace(&mut self.elements, new_elements);
    }
}

impl<IterFn, State, Item, Cons> UIHandler for MultiElement<IterFn, State, Item, Cons>
    where IterFn: FnMut(Box<dyn for<'a> FnMut(&mut State, &'a mut Item)>),
          Cons: FnMut(bool, &mut State, &mut Item) -> Option<Box<dyn UIHandler>> + 'static,
          State: Debug,
          Item: UIIdentifier + Debug,
{
    unsafe fn handle(&mut self, event: &Event) -> bool {
        match event {
            Event::PreRender => {
                self.update_elements();
            }
            _ => {}
        }

        let mut handled = false;
        for el in self.elements.values_mut() {
            handled = el.handle(event);
        }

        handled
    }

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        for el in self.elements.values_mut() {
            if el.should_render(render_pass) {
                return true;
            }
        }
        false
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        None // TODO
    }
}

pub struct Element {
    bounds: Changing<Vec4>,
    handler: Arc<Mutex<Box<dyn FnMut(&mut Self, &Event)>>>,
    should_render_fn: Arc<Mutex<Box<dyn FnMut(&mut Self, &RenderPass) -> bool>>>,
    hovering: bool,
    children: Vec<Box<dyn UIHandler>>,
    pub draggable: bool,
    pub scrollable: bool,
    scroll: Changing<(f32, f32)>,
    dragging: (bool, Vec2),
    has_rendered: bool,
    animations: AnimationRegistry,
}

impl Element {
    pub fn new<B, H>(vec4: B, draggable: bool, handler: H, children: Vec<Box<dyn UIHandler>>) -> Element
    where B: Into<Vec4>,
          H: FnMut(&mut Self, &Event) + 'static,
    {
        let b = vec4.into();
        Element {
            bounds: Changing::new(b.clone()),
            handler: Arc::new(Mutex::new(Box::new(handler))),
            should_render_fn: Arc::new(Mutex::new(Box::new(|_, _| false))),
            hovering: false,
            children,
            draggable,
            scrollable: false,
            scroll: Changing::new((0.0, 0.0)),
            dragging: (false, Vec2::new(0.0, 0.0)),
            has_rendered: false,
            animations: AnimationRegistry::new(),
        }
    }
    pub fn text(mut fr: FontRenderer, size: f32, text: impl ToString, pos: impl Into<Vec2>, color: impl ToColor) -> Element {
        let pos = pos.into();
        let text = text.to_string();
        let color = color.to_color();

        let text_c = text.clone();
        let builder = ElementBuilder::new()
            .vec4(Vec4::xywh(pos.x, pos.y, 0.0, 0.0))
            .handler(move |el, event| unsafe {
                match event {
                    Event::Render(pass) => {
                        match pass {
                            RenderPass::Main => {
                                let (_, b) = fr.draw_string((size, &text_c, color), el.bounds.current().top_left());
                                el.bounds().set_width(b.width());
                                el.bounds().set_height(b.height());
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
    pub fn set_bounds(&mut self, vec4: Vec4) {
        self.bounds.set(vec4);
    }
    pub fn bounds(&mut self) -> &mut Vec4 {
        self.bounds.current()
    }
    pub fn hovering(&self) -> bool {
        self.hovering
    }
    pub fn scroll(&mut self) -> &mut Changing<(f32, f32)> {
        &mut self.scroll
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
            // Event::Render(pass) => {
                // match pass {
                //     RenderPass::Main => self.vec4().draw_vec4(0xffffffff),
                //     _ => {}
                // }
            // }
            Event::PostRender => {
                self.has_rendered = true;
                self.scroll().update();
                self.bounds.update();
            }
            _ => {}
        }

        // Arc mutex so that can be called with self ref
        let h = self.handler.clone();
        (h.lock())(self, event);

        // Translate child positions, which also offsets mouse correctly
        context().renderer().stack().push(State::Translate(self.bounds().x(), self.bounds().y()));
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
                    let mut updated = *self.scroll.current();
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
        if !self.has_rendered || self.bounds.changed() || self.scroll.changed()  {
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
    pub fn should_render<H: FnMut(&mut Element, &RenderPass) -> bool + 'static>(mut self, should_render: H) -> Self {
        self.element.should_render_fn = Arc::new(Mutex::new(Box::new(should_render))); self
    }
    pub fn child<C: UIHandler + 'static>(mut self, child: C) -> Self {
        self.element.children.push(Box::new(child)); self
    }
    pub fn vec4(mut self, vec4: Vec4) -> Self {
        self.element.bounds.set(vec4); self
    }
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.element.draggable = draggable; self
    }
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.element.scrollable = scrollable; self
    }
    pub fn animations(&mut self) -> &mut AnimationRegistry {
        &mut self.element.animations
    }
    pub fn build(self) -> Element {
        self.element
    }
}