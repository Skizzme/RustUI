use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
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
use crate::components::framework::animation::{Animation, AnimationRegistry};

pub struct Framework {
    pub(super) current_screen: Box<dyn ScreenTrait>,
    screen_passes: HashMap<RenderPass, u32>,
    element_passes: HashMap<RenderPass, u32>,
    layers: Vec<Layer>,
    created_at: Instant,
    last_pre_render: Instant,
    pre_delta: f32,
}

impl Framework {
    pub unsafe fn new(fb_manager: &mut FramebufferManager, width: i32, height: i32,) -> Self {
        let mut fr = Framework {
            current_screen: Box::new(DefaultScreen::new()),
            screen_passes: HashMap::new(),
            element_passes: HashMap::new(),
            layers: vec![],
            created_at: Instant::now(),
            last_pre_render: Instant::now(),
            pre_delta: 0.0,
        };
        fr.set_screen(DefaultScreen::new());
        fr
    }

    pub unsafe fn on_resize(&mut self) {
        self.created_at = Instant::now();
    }

    pub unsafe fn should_render(&mut self, layer: u32, render_pass: &RenderPass) -> bool {
        self.layers.get_mut(layer as usize).unwrap().should_render(render_pass)
    }

    pub unsafe fn should_render_pass(&mut self, render_pass: &RenderPass) -> bool {
        if self.created_at_elapsed() || self.current_screen.should_render(render_pass) {
            return true
        }
        for i in 0..self.layers.len() {
            if self.should_render(i as u32, render_pass) {
                return true
            }
        }
        false
    }

    fn created_at_elapsed(&self) -> bool {
        self.created_at.elapsed().as_secs_f64() < 1.0 // TODO why is this necessary?
    }

    pub unsafe fn should_render_all(&mut self) -> bool {
        if self.created_at_elapsed() {
            return true
        }
        for rp in RenderPass::all() {
            if self.should_render_pass(&rp) {
                return true;
            }
            // for i in 0..self.layers.len() {
            //     if self.should_render(i as u32, &rp) {
            //         return true
            //     }
            // }
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
            Event::PreRender => {
                self.pre_delta = self.last_pre_render.elapsed().as_secs_f64() as f32;
                self.last_pre_render = Instant::now();
            }
            Event::Render(pass) => {
                let (parent_fb, parent_tex) = self.screen_pass_fb(pass).bind();
                if self.current_screen.should_render(pass) || self.created_at_elapsed() {
                    Framebuffer::clear_current();
                    // if parent != 0 {
                    //     context().fb_manager().fb(parent as u32).copy(fb.id());
                    // }

                    self.current_screen.handle(&event);

                    self.screen_pass_fb(pass).unbind();
                }

                self.screen_pass_fb(pass).copy_bind(parent_fb as u32, parent_tex as u32);
            },
            _ => self.current_screen.handle(&event),
        }
        for layer in &mut self.layers {
            match &event {
                Event::Render(pass) => {
                    let (mut parent_fb, mut parent_tex) = (0, 0);
                    {
                        let layer_fb = layer.fb(pass);
                        (parent_fb, parent_tex) = layer_fb.bind();
                    }

                    if layer.should_render(pass) {
                        // println!("did render layer {:?}", pass);
                        Framebuffer::clear_current();

                        for e in layer.elements() {
                            e.handle(&event);
                        }

                    }
                    let layer_fb = layer.fb(pass);
                    layer_fb.unbind();
                    layer_fb.copy_bind(parent_fb as u32, parent_tex as u32);
                },
                _ => {
                    for e in layer.elements() {
                        e.handle(&event);
                        match event {
                            Event::PostRender => {
                                match e.animations() {
                                    None => {}
                                    Some(reg) => { reg.post(); }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
        // match &event {
        //     Event::PostRender => self.created_at_elapsed() = false,
        //     _ => {}
        // }
    }

    pub unsafe fn set_screen<S>(&mut self, screen: S) where S: ScreenTrait + 'static {
        self.reset();
        self.current_screen = Box::new(screen);
        self.layers = self.current_screen.init();
    }
    pub fn pre_delta(&self) -> f32 {
        self.pre_delta
    }
}