use std::collections::HashMap;
use crate::components::context::context;
use crate::components::framework::element::{Element, UIHandler};
use crate::components::framework::event::RenderPass;
use crate::components::wrapper::framebuffer::Framebuffer;
use crate::gl_binds::gl11::RGBA;

pub struct Layer {
    framebuffers: HashMap<RenderPass, u32>,
    elements: Vec<Box<dyn UIHandler>>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            framebuffers: HashMap::new(),
            elements: vec![],
        }
    }

    pub unsafe fn fb(&mut self, render_pass: &RenderPass) -> &mut Framebuffer{
        if !self.framebuffers.contains_key(render_pass) {
            self.framebuffers.insert(render_pass.clone(), context().fb_manager().create_fb(RGBA).unwrap());
        }
        let fb_id = *self.framebuffers.get(render_pass).unwrap();
        context().fb_manager().fb(fb_id)
    }

    pub unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        for e in &mut self.elements {
            if e.should_render(render_pass) {
                return true;
            }
        }
        false
    }

    pub unsafe fn add_element(&mut self, el: Element) {
        self.elements.push(Box::new(el));
    }

    pub fn elements(&mut self) -> &mut Vec<Box<dyn UIHandler>> {
        &mut self.elements
    }
}