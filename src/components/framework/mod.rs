use std::collections::HashMap;
use std::time::Instant;
use gl::Finish;
use crate::components::context::context;
use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::framework::layer::Layer;
use crate::components::framework::screen::{DefaultScreen, ScreenTrait};
use crate::components::framework::state::{ChangingRegistry, UnchangingRegistry};
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;
use crate::components::wrapper::framebuffer::Framebuffer;
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::gl11::RGBA;
use crate::gl_binds::gl30::{ActiveTexture, BindFramebuffer, BindTexture, Disable, Enable, BLEND, FRAMEBUFFER, TEXTURE0, TEXTURE1, TEXTURE2, TEXTURE_2D};

pub mod screen;
pub mod event;
pub mod element;
pub mod layer;
pub mod animation;
pub mod state;
pub mod changing;

pub struct Framework {
    pub(super) current_screen: Box<dyn ScreenTrait>,
    screen_animations: AnimationRegistry,
    screen_passes: HashMap<RenderPass, u32>,
    element_passes: HashMap<RenderPass, u32>,
    layers: Vec<Layer>,
    created_at: Instant,
    last_pre_render: Instant,
    style: UnchangingRegistry,
    states: ChangingRegistry,

    current_layer_pass: (RenderPass, usize),

    pre_delta: f32,
}

impl Framework {
    pub unsafe fn new() -> Self {
        let mut fr = Framework {
            current_screen: Box::new(DefaultScreen::new()),
            screen_animations: AnimationRegistry::new(),
            screen_passes: HashMap::new(),
            element_passes: HashMap::new(),
            layers: vec![],
            created_at: Instant::now(),
            last_pre_render: Instant::now(),
            style: UnchangingRegistry::new(),
            states: ChangingRegistry::new(),
            current_layer_pass: (RenderPass::Main, 0),
            pre_delta: 0.0,
        };
        fr.set_screen(DefaultScreen::new());
        fr
    }

    pub unsafe fn mark_layer_dirty(&mut self, area: impl Into<Vec4>) {
        self.layers.get_mut(self.current_layer_pass.1).unwrap().mark_dirty(&self.current_layer_pass.0, area);
    }

    pub fn set_styles(&mut self, style: UnchangingRegistry) {
        self.style = style;
    }

    pub fn states(&mut self) -> &mut ChangingRegistry {
        &mut self.states
    }

    pub fn style(&self) -> &UnchangingRegistry {
        &self.style
    }

    pub unsafe fn on_resize(&mut self, width: f32, height: f32) {
        self.created_at = Instant::now();
        self.event(Event::Resize(width as f32, height as f32));
    }

    pub unsafe fn should_render(&mut self, layer: u32, render_pass: &RenderPass) -> bool {
        self.layers.get_mut(layer as usize).unwrap().should_render(render_pass)
    }

    pub unsafe fn should_render_pass(&mut self, render_pass: &RenderPass) -> bool {
        if self.created_at_elapsed() || self.current_screen.should_render(render_pass) || self.screen_animations.has_changed() {
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
        if self.created_at_elapsed() || self.screen_animations.has_changed() {
            return true
        }
        for rp in RenderPass::all() {
            if self.should_render_pass(&rp) {
                return true;
            }
        }
        false
    }

    fn reset(&mut self) {
        self.layers = Vec::new();
        self.created_at = Instant::now();
        self.screen_animations = AnimationRegistry::new();
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

    pub unsafe fn copy_bind_rects(&self, screen_pass_fb_tex: u32, target_fb: u32, target_tex: u32) {
        Disable(BLEND);
        BindFramebuffer(FRAMEBUFFER, target_fb);

        context().renderer().blend_shader.bind();
        context().renderer().blend_shader.u_put_int("u_bottom_tex", vec![2]);
        context().renderer().blend_shader.u_put_int("u_top_tex", vec![1]);

        ActiveTexture(TEXTURE2);
        BindTexture(TEXTURE_2D, target_tex);

        ActiveTexture(TEXTURE1);
        BindTexture(TEXTURE_2D, screen_pass_fb_tex);

        ActiveTexture(TEXTURE0);

        Texture::unbind();
        context().renderer().draw_screen_rect_flipped();
        Shader::unbind();
        Enable(BLEND);
    }

    pub unsafe fn event(&mut self, event: Event) {
        match &event {
            Event::PreRender => {
                self.pre_delta = self.last_pre_render.elapsed().as_secs_f64() as f32;
                self.last_pre_render = Instant::now();
                self.current_screen.handle(&event);
            }
            Event::Render(pass) => {
                let (parent_fb, parent_tex) = self.screen_pass_fb(pass).bind();

                if self.current_screen.should_render(pass) || self.created_at_elapsed() || self.screen_animations.has_changed() {
                    Framebuffer::clear_current();
                    // if parent != 0 {
                    //     context().fb_manager().fb(parent as u32).copy(fb.id());
                    // }

                    self.current_screen.handle(&event);

                    self.screen_pass_fb(pass).unbind();
                }

                // At 4K this copy takes ~0.2ms on GPU and ~2.0ms on iGPU. Likely to have a very big performance impact on lower power integrated graphics
                // At 4K, all copies per frame is around 5.4ms on iGPU and 0.6ms on GPU
                // TODO dirty rects https://trello.com/c/LEwMbrmE
                let id = self.screen_pass_fb(pass).texture_id();
                self.copy_bind_rects(id, parent_fb as u32, parent_tex as u32);
                // BindFramebuffer(FRAMEBUFFER, parent_fb as u32);
            },
            _ => self.current_screen.handle(&event),
        }
        let force = self.created_at_elapsed();
        for layer in &mut self.layers {
            match &event {
                Event::Render(pass) => {
                    let (mut parent_fb, mut parent_tex) = (0, 0);
                    {
                        let layer_fb = layer.fb(pass);
                        (parent_fb, parent_tex) = layer_fb.bind();
                    }
                    {
                        // println!("{}", layer.should_render(pass));
                        if layer.should_render(pass) || force {
                            // println!("did render layer {:?}", pass);
                            Framebuffer::clear_current();

                            for e in layer.elements() {
                                e.handle(&event);
                            }
                        }
                        let layer_fb = layer.fb(pass);
                        layer_fb.unbind();
                    }
                    layer.copy_bind_rects(pass, parent_fb as u32, parent_tex as u32);
                    // layer_fb.copy_bind(parent_fb as u32, parent_tex as u32);
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
        match &event {
            Event::PostRender => {
                self.screen_animations.post();
                self.states.update();
            },
            _ => {}
        }
    }

    pub unsafe fn set_screen<S>(&mut self, screen: S) where S: ScreenTrait + 'static {
        self.reset();
        self.current_screen = Box::new(screen);
        self.layers = self.current_screen.init();
    }

    pub fn screen_animations(&mut self) -> &mut AnimationRegistry {
        &mut self.screen_animations
    }

    pub fn pre_delta(&self) -> f32 {
        self.pre_delta
    }
}