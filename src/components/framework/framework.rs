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
    screen_fb: u32,
    passes_fb: HashMap<RenderPass, u32>,
    layers: Vec<Layer>,
    first_render: bool,
}

impl Framework {
    pub unsafe fn new(fb_manager: &mut FramebufferManager, width: i32, height: i32,) -> Self {
        let mut fr = Framework {
            current_screen: Box::new(DefaultScreen::new()),
            screen_fb: 0,
            passes_fb: HashMap::new(),
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

    pub unsafe fn pass_fb(&mut self, pass: &RenderPass) -> &mut Framebuffer {
        if !self.passes_fb.contains_key(&pass) {
            self.passes_fb.insert(pass.clone(), context().fb_manager().create_fb(RGBA).unwrap());
        }
        context().fb_manager().fb(*self.passes_fb.get(&pass).unwrap())
    }

    pub unsafe fn event(&mut self, event: Event) {
        match &event {
            Event::Render(pass) => {
                if self.screen_fb == 0 {
                    self.screen_fb = context().fb_manager().create_fb(RGBA).unwrap();
                }
                let fb = context().fb_manager().fb(self.screen_fb);
                let parent = fb.bind();
                if self.current_screen.should_render() || self.first_render {
                    fb.clear();
                    if parent != 0 {
                        context().fb_manager().fb(parent as u32).copy(fb.id());
                    }
                    println!("red");

                    self.current_screen.handle(&event);

                    fb.unbind();
                }
                fb.copy_to_parent();
            },
            _ => self.current_screen.handle(&event),
        }
        for layer in &mut self.layers {
            match &event {
                Event::Render(pass) => {
                    {
                        let layer_fb = layer.fb(pass);
                        let parent = layer_fb.bind();
                        layer_fb.clear();
                        if parent != 0 {
                            context().fb_manager().fb(parent as u32).copy(layer_fb.id());
                        }
                    }

                    for e in layer.elements() {
                        e.handle(&event);
                    }

                    let layer_fb = layer.fb(pass);
                    layer_fb.unbind();
                    layer_fb.copy_to_parent();
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