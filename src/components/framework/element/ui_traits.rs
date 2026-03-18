use std::cell::RefCell;
use std::rc::Rc;
use rand::{RngCore, thread_rng};

use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::spatial::vec4::Vec4;

pub trait UIHandler {
    unsafe fn handle(&mut self, event: &Event) -> bool;
    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool;
    fn animations(&mut self) -> Option<AnimationRegistry>;
    fn bounds(&self) -> Vec4;
    fn min_bounds(&self) -> Vec4;
    fn max_bounds(&self) -> Vec4;
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
    unsafe fn handle(&mut self, event: &Event) -> bool {
        self.handler.borrow_mut().handle(event)
    }

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        self.handler.borrow_mut().should_render(render_pass)
    }

    fn animations(&mut self) -> Option<AnimationRegistry> {
        self.handler.borrow_mut().animations()
    }

    fn bounds(&self) -> Vec4 {
        self.handler.borrow().bounds()
    }

    fn min_bounds(&self) -> Vec4 {
        self.handler.borrow().min_bounds()
    }

    fn max_bounds(&self) -> Vec4 {
        self.handler.borrow().max_bounds()
    }
}


pub fn random_id() -> u64 {
    thread_rng().next_u64()
}