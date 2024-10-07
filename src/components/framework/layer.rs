use std::collections::HashMap;
use crate::components::context::context;
use crate::components::framework::element::UIHandler;
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

    pub unsafe fn bind(&mut self, render_pass: &RenderPass) {
        if !self.framebuffers.contains_key(render_pass) {
            self.framebuffers.insert(render_pass.clone(), context().fb_manager().create_fb(RGBA).unwrap());
        }
        let fb_id = *self.framebuffers.get(render_pass).unwrap();
        context().fb_manager().fb(fb_id).bind();
    }

    pub fn elements(&mut self) -> &mut Vec<Box<dyn UIHandler>> {
        &mut self.elements
    }
}