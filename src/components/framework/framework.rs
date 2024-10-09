use std::collections::HashMap;
use std::time::Instant;
use glfw::WindowEvent;
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::framework::element::{Element, UIHandler};
use crate::components::framework::event::{Event, RenderPass};
use crate::components::framework::layer::Layer;
use crate::components::framework::screen::{DefaultScreen, ScreenTrait};
use crate::components::render::stack::State;
use crate::components::wrapper::framebuffer::{Framebuffer, FramebufferManager};
use crate::gl_binds::gl11::{ALPHA, Enable, Finish, RGBA};
use crate::gl_binds::gl30::{BindFramebuffer, FRAMEBUFFER};

pub struct Framework {
    pub(super) current_screen: Box<dyn ScreenTrait>,
    screen_passes: HashMap<RenderPass, u32>,
    element_passes: HashMap<RenderPass, u32>,
    layers: Vec<Layer>,
    first_render: bool,
}

impl Framework {
    pub unsafe fn new(fb_manager: &mut FramebufferManager, width: i32, height: i32,) -> Self {
        let mut fr = Framework {
            current_screen: Box::new(DefaultScreen::new()),
            screen_passes: HashMap::new(),
            element_passes: HashMap::new(),
            layers: vec![],
            first_render: true,
        };
        fr.set_screen(DefaultScreen::new());
        fr
    }

    pub unsafe fn on_resize(&mut self) {
        self.first_render = true;
    }

    pub unsafe fn should_render(&mut self, layer: u32, render_pass: &RenderPass) -> bool {
        self.layers.get_mut(layer as usize).unwrap().should_render(render_pass)
    }

    pub unsafe fn should_render_pass(&mut self, render_pass: &RenderPass) -> bool {
        if self.first_render || self.current_screen.should_render() {
            return true
        }
        for i in 0..self.layers.len() {
            if self.should_render(i as u32, render_pass) {
                return true
            }
        }
        false
    }

    pub unsafe fn should_render_all(&mut self) -> bool {
        if self.first_render || self.current_screen.should_render() {
            return true
        }
        for i in 0..self.layers.len() {
            for rp in RenderPass::all() {
                if self.should_render(i as u32, &rp) {
                    return true
                }
            }
        }
        false
    }

    fn reset(&mut self) {
        self.layers = Vec::new();
    }

    pub fn current_screen(&mut self) -> &mut Box<dyn ScreenTrait> {
        &mut self.current_screen
    }

    pub unsafe fn element_pass_fb(&mut self, pass: &RenderPass) -> &mut Framebuffer {
        if !self.element_passes.contains_key(&pass) {
            self.element_passes.insert(pass.clone(), context().fb_manager().create_fb(RGBA).unwrap());
        }
        context().fb_manager().fb(*self.element_passes.get(&pass).unwrap())
    }

    pub unsafe fn screen_pass_fb(&mut self, pass: &RenderPass) -> &mut Framebuffer {
        if !self.screen_passes.contains_key(&pass) {
            self.screen_passes.insert(pass.clone(), context().fb_manager().create_fb(RGBA).unwrap());
        }
        context().fb_manager().fb(*self.screen_passes.get(&pass).unwrap())
    }

    pub unsafe fn event(&mut self, event: Event) {
        match &event {
            Event::Render(pass) => {
                let (parent_fb, parent_tex) = self.screen_pass_fb(pass).bind();
                if self.current_screen.should_render() || self.first_render {
                    Framebuffer::clear_current();
                    // if parent != 0 {
                    //     context().fb_manager().fb(parent as u32).copy(fb.id());
                    // }
                    println!("red");

                    self.current_screen.handle(&event);

                    self.screen_pass_fb(pass).unbind();
                }
                println!("parent {} {}", parent_fb, parent_tex);
                self.screen_pass_fb(pass).copy_bind(parent_fb as u32, parent_tex as u32);
            },
            _ => self.current_screen.handle(&event),
        }
        for layer in &mut self.layers {
            match &event {
                Event::Render(pass) => {
                    let (mut parent_fb, mut parent_tex) = (0,0);
                    {
                        let layer_fb = layer.fb(pass);
                        (parent_fb, parent_tex) = layer_fb.bind();
                        println!("layer: {} {}", parent_fb, parent_tex);
                        Framebuffer::clear_current();
                        // if parent != 0 {
                        //     context().fb_manager().fb(parent as u32).copy(layer_fb.id());
                        // }
                    }

                    for e in layer.elements() {
                        e.handle(&event);
                    }

                    let layer_fb = layer.fb(pass);
                    layer_fb.unbind();
                    layer_fb.copy_bind(parent_fb as u32, parent_tex as u32);
                },
                _ => {
                    for e in layer.elements() {
                        e.handle(&event);
                    }
                }
            }
        }
        match &event {
            Event::PostRender => self.first_render = false,
            _ => {}
        }
    }

    pub unsafe fn set_screen<S>(&mut self, screen: S) where S: ScreenTrait + 'static {
        self.reset();
        self.current_screen = Box::new(screen);
        self.layers = self.current_screen.init();
    }
}