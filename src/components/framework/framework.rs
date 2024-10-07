use glfw::WindowEvent;
use crate::components::context::context;
use crate::components::framework::element::{Element, UIHandler};
use crate::components::framework::event::Event;
use crate::components::framework::layer::Layer;
use crate::components::framework::screen::{DefaultScreen, ScreenTrait};
use crate::components::wrapper::framebuffer::FramebufferManager;
use crate::gl_binds::gl11::RGBA;

pub struct Framework {
    pub(super) current_screen: Box<dyn ScreenTrait>,
    screen_fb: u32,
    layers: Vec<Layer>,
}

impl Framework {
    pub unsafe fn new(fb_manager: &mut FramebufferManager) -> Self {
        let mut fr = Framework {
            current_screen: Box::new(DefaultScreen::new()),
            screen_fb: 0,
            layers: vec![],
        };
        fr.screen_fb = fb_manager.create_fb(RGBA).unwrap();
        fr.set_screen(DefaultScreen::new());
        fr
    }

    pub unsafe fn should_render(&mut self, layer: u32) -> bool {
        let mut res = self.current_screen.should_render();
        for e in &mut self.layers.get_mut(layer as usize).unwrap() {
            if res {
                break;
            }
            res = e.should_render();
        }

        res
    }

    fn reset(&mut self) {
        self.layers = Vec::new();
    }

    pub fn current_screen(&mut self) -> &mut Box<dyn ScreenTrait> {
        &mut self.current_screen
    }

    pub unsafe fn event(&mut self, event: Event) {
        match event {
            Event::Render(_) => context().fb_manager().fb(self.screen_fb).bind(),
            _ => {}
        }
        self.current_screen.handle(&event);
        for layer in &mut self.layers {
            match &event {
                Event::Render(pass) => layer.bind(pass),
                _ => {}
            }
            for e in layer.elements() {
                e.handle(&event);
            }
        }
    }

    pub unsafe fn set_screen<S>(&mut self, screen: S) where S: ScreenTrait + 'static {
        self.reset();
        self.current_screen = Box::new(screen);
        // self.layers = self.current_screen.init();
    }
}