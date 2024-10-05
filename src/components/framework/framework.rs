use glfw::WindowEvent;
use crate::components::framework::element::Element;
use crate::components::framework::event::Event;
use crate::components::framework::screen::{DefaultScreen, ScreenTrait};

pub struct Framework {
    pub(super) current_screen: Box<dyn ScreenTrait>,
    elements: Vec<Element>,
}

impl Framework {
    pub unsafe fn new() -> Self {
        let mut fr = Framework {
            current_screen: Box::new(DefaultScreen {text: "framework".to_string()}),
            elements: vec![],
        };
        fr.set_screen(DefaultScreen{text: "framework".to_string()});
        fr
    }
    fn reset(&mut self) {
        self.elements = Vec::new();
    }

    pub fn current_screen(&mut self) -> &mut Box<dyn ScreenTrait> {
        &mut self.current_screen
    }

    pub unsafe fn event(&mut self, event: Event) {
        self.current_screen.event(&event);
        for e in &mut self.elements {
            e.handle(&event);
        }
    }

    pub unsafe fn set_screen<S>(&mut self, screen: S) where S: ScreenTrait + 'static {
        self.reset();
        self.current_screen = Box::new(screen);
        self.elements = self.current_screen.register_elements();
        println!("{}", self.elements.len());
    }
}