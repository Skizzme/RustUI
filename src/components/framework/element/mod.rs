use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use glfw::{Action, MouseButton};
use image::codecs::png::CompressionType::Default;
use parking_lot::Mutex;

use crate::components::context::context;
use crate::components::framework::animation::{AnimationRef, AnimationRegistry};
use crate::components::framework::changing::Changing;
use crate::components::framework::ui_traits::{TickResult, UIHandler, UIIdentifier};
use crate::components::framework::event::{Event, EventResult, RenderPass};
use crate::components::framework::layout::{LayoutContext, LayoutEvent};
use crate::components::framework::ui_traits;
use crate::components::render::color::ToColor;
use crate::components::render::stack::State;
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;

pub mod comp_element;
pub mod container;

pub struct Element {
    id: u64,
    bounds: Changing<Vec4>,
    hovering: bool,

    layout_context: LayoutContext,

    pub active: bool,
    last_active: bool,
    pub draggable: bool,
    pub scrollable: bool,

    scroll: Changing<(f32, f32)>,
    dragging: (bool, Vec2<f32>),
    has_rendered: bool,
    animations: AnimationRegistry,

    handler: Arc<Mutex<Box<dyn FnMut(&mut Self, &Event)>>>,
    render_handler: Option<Arc<Mutex<Box<dyn FnMut(&mut Self, &RenderPass)>>>>,
    tick_fn: Arc<Mutex<Box<dyn FnMut(&mut Self, &RenderPass) -> TickResult>>>,
    active_fn: Option<Box<dyn FnMut() -> bool>>,
}

impl Element {
    pub fn new<B, H>(vec4: B, draggable: bool, layout_context: LayoutContext, handler: H,) -> Element
        where B: Into<Vec4>,
              H: FnMut(&mut Self, &Event) + 'static,
    {
        let b = vec4.into();
        Element {
            id: ui_traits::random_id(),
            bounds: Changing::new(b.clone()),
            handler: Arc::new(Mutex::new(Box::new(handler))),
            render_handler: None,
            tick_fn: Arc::new(Mutex::new(Box::new(|_, _| TickResult::Valid))),
            hovering: false,
            layout_context,
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
    // pub fn text(mut fr: FontRenderer, size: f32, text: impl ToString, pos: impl Into<Vec2<f32>>, color: impl ToColor) -> Element {
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
    unsafe fn handle(&mut self, event: &Event) -> EventResult {
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
            Event::Layout(stage) => {
                self.dispatch_event(event);
                return match stage {
                    LayoutEvent::FitWidth => {
                        self.bounds.current_mut().width = self.layout_context.min_size.x;
                        EventResult::Ok
                    }
                    LayoutEvent::GrowWidth(v) => {
                        self.bounds.current_mut().width += *v;
                        if let Some(max) = self.layout_context.max_size {
                            if self.bounds.current_mut().width() > max.x {
                                self.bounds.current_mut().width = max.x;
                                return EventResult::LayoutError;
                            }
                        }
                        EventResult::Ok
                    }
                    LayoutEvent::OptimizeSize(v) => {
                        EventResult::LayoutError
                    }
                    LayoutEvent::FitHeight => {
                        self.bounds.current_mut().height = self.layout_context.min_size.y;
                        EventResult::Ok
                    }
                    LayoutEvent::GrowHeight(v) => {
                        self.bounds.current_mut().height += *v;
                        if let Some(max) = self.layout_context.max_size {
                            if self.bounds.current_mut().height() > max.y {
                                self.bounds.current_mut().height = max.y;
                                return EventResult::LayoutError;
                            }
                        }
                        EventResult::Ok
                    }
                    LayoutEvent::Position(pos) => {
                        self.bounds.current_mut().set_pos(*pos);
                        EventResult::Ok
                    }
                }
            }
            _ => {}
        }

        if !self.active {
            return EventResult::Ok;
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
        if handled {
            EventResult::Used
        } else {
            EventResult::Ok
        }
    }


    unsafe fn tick(&mut self, rp: &RenderPass) -> TickResult {
        let changed_active = self.last_active != self.active;
        if !self.active && !changed_active {
            return TickResult::RedrawLayout;
        }
        if !self.has_rendered || self.scroll.changed() || self.bounds.changed() || changed_active {
            return TickResult::RedrawLayout;
        }

        let fn_ref = self.tick_fn.clone();
        // println!("check call");
        let v = (fn_ref.lock())(self, rp);
        if !v.is_valid() {
            return v;
        }

        for a in self.animations().unwrap().all() {
            if a.borrow().has_changed() {
                return TickResult::RedrawLayout;
            }
        }

        TickResult::Valid
    }

    fn animations(&mut self) -> Option<AnimationRegistry> {
        Some(self.animations.clone())
    }

    fn bounds(&self) -> Vec4 {
        self.bounds.current().clone()
    }

    fn layout_context(&self) -> LayoutContext {
        self.layout_context.clone()
    }
}

impl UIIdentifier for Element {
    fn ui_id(&self) -> u64 {
        self.id
    }
}

#[macro_export]
macro_rules! element {
    (
        layout: { $($layout:tt)* },
        $closure:expr $(,)?
    ) => { // |$element:ident, $event:ident| $body:block
        ElementBuilder::new()
            .layout_context(
                LayoutContext {
                    $($layout)*
                    ..Default::default()
                }
            )
            .handler($closure)
    };

    (
        $closure:expr $(,)?
    ) => { // |$element:ident, $event:ident| $body:block
        ElementBuilder::new()
            .handler($closure)
    };
}

pub struct ElementBuilder {
    element: Element,
}

impl ElementBuilder {
    pub fn new() -> Self {
        ElementBuilder {
            element: Element::new(Vec4::ltrb(0.0, 0.0, 0.0, 0.0), false, LayoutContext::new(), |_, _| {})
        }
    }

    pub fn layout_context(mut self, layout_context: LayoutContext) -> Self {
        self.element.layout_context = layout_context;
        self
    }

    pub fn handler<H: FnMut(&mut Element, &Event) + 'static>(mut self, handler: H) -> Self {
        self.element.handler = Arc::new(Mutex::new(Box::new(handler)));
        self
    }
    pub fn render_handler<R: FnMut(&mut Element, &RenderPass) + 'static>(mut self, render_handler: R) -> Self {
        self.element.render_handler = Some(Arc::new(Mutex::new(Box::new(render_handler))));
        self
    }
    pub fn tick<H: FnMut(&mut Element, &RenderPass) -> TickResult + 'static>(mut self, should_render: H) -> Self {
        self.element.tick_fn = Arc::new(Mutex::new(Box::new(should_render)));
        self
    }

    pub fn bounds(mut self, vec4: impl Into<Vec4>) -> Self {
        self.element.bounds.set(vec4.into());
        self
    }
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.element.draggable = draggable;
        self
    }
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.element.scrollable = scrollable;
        self
    }
    pub fn active(mut self, active: bool) -> Self {
        self.element.active = active;
        self
    }
    pub fn active_fn<Fn: FnMut() -> bool + 'static>(mut self, active_fn: Option<Fn>) -> Self {
        self.element.set_active_fn(active_fn);
        self
    }
    pub fn register_animations(mut self, anims: Vec<AnimationRef>) -> Self {
        for a in anims {
            self.element.animations.register(a);
        }
        self
    }
    pub fn animations(&mut self) -> &mut AnimationRegistry {
        &mut self.element.animations
    }
    pub fn build(self) -> Element {
        self.element
    }
}