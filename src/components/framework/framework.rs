use glfw::WindowEvent;
use crate::components::framework::element::{Element, UIHandler};
use crate::components::framework::event::Event;
use crate::components::framework::screen::{DefaultScreen, ScreenTrait};

pub struct Framework {
    pub(super) current_screen: Box<dyn ScreenTrait>,
    elements: Vec<Box<dyn UIHandler>>,
}

impl Framework {
    pub unsafe fn new() -> Self {
        let mut fr = Framework {
            current_screen: Box::new(DefaultScreen::new()),
            elements: vec![],
        };
        fr.set_screen(DefaultScreen::new());
        fr
    }

    pub unsafe fn should_render(&mut self) -> bool {
        let mut res = self.current_screen.should_render();
        for e in &mut self.elements {
            if res {
                break;
            }
            res = e.should_render();
        }

        res
    }

    fn reset(&mut self) {
        self.elements = Vec::new();
    }

    pub fn current_screen(&mut self) -> &mut Box<dyn ScreenTrait> {
        &mut self.current_screen
    }

    pub unsafe fn event(&mut self, event: Event) {
        self.current_screen.handle(&event);
        for e in &mut self.elements {
            e.handle(&event);
        }
    }

    pub unsafe fn set_screen<S>(&mut self, screen: S) where S: ScreenTrait + 'static {
        self.reset();
        self.current_screen = Box::new(screen);
        self.elements = self.current_screen.init();
        println!("{}", self.elements.len());
    }
}