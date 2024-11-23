use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::event::{Event, RenderPass};

pub trait UIHandler {
    unsafe fn handle(&mut self, event: &Event) -> bool;
    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool;
    fn animations(&mut self) -> Option<&mut AnimationRegistry>;
}

pub trait UIIdentifier {
    fn ui_id(&self) -> u64;
}