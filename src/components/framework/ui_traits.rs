use std::cell::RefCell;
use std::rc::Rc;
use rand::{RngCore, thread_rng};

use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::event::{Event, EventResult, RenderPass};
use crate::components::framework::layout::LayoutContext;
use crate::components::spatial::vec4::Vec4;

pub enum TickResult {
    /// Nothing needs redrawing
    Valid,
    /// Only redraws the current layout
    Redraw,
    /// Redraws and re-calculates the layout
    RedrawLayout,
}

impl TickResult {
    pub fn is_valid(&self) -> bool {
        match self {
            TickResult::Valid => true,
            _ => false,
        }
    }
}

pub trait UIHandler {
    unsafe fn handle(&mut self, event: &Event) -> EventResult;
    unsafe fn tick(&mut self, render_pass: &RenderPass) -> TickResult;
    fn animations(&mut self) -> Option<AnimationRegistry>;
    fn bounds(&self) -> Vec4;
    fn layout_context(&self) -> LayoutContext;
}

pub trait UIIdentifier {
    fn ui_id(&self) -> u64;
}

#[derive(Clone)]
pub struct UIHandlerRef {
    handler: Rc<RefCell<dyn UIHandler>>
}

impl UIHandlerRef {
    pub fn new<H: UIHandler + 'static>(handler: H) -> (Self, Rc<RefCell<H>>) {
        let cell = Rc::new(RefCell::new(handler));
        (
            UIHandlerRef {
                handler: cell.clone(),
            },
            cell
        )
    }
}

impl UIHandler for UIHandlerRef {
    unsafe fn handle(&mut self, event: &Event) -> EventResult {
        self.handler.borrow_mut().handle(event)
    }

    unsafe fn tick(&mut self, render_pass: &RenderPass) -> TickResult {
        self.handler.borrow_mut().tick(render_pass)
    }

    fn animations(&mut self) -> Option<AnimationRegistry> {
        self.handler.borrow_mut().animations()
    }

    fn bounds(&self) -> Vec4 {
        self.handler.borrow().bounds()
    }

    fn layout_context(&self) -> LayoutContext {
        self.handler.borrow().layout_context().clone()
    }
}


pub fn random_id() -> u64 {
    thread_rng().next_u64()
}