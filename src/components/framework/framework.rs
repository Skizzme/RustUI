use image::Frame;
use crate::components::framework::screen::{DefaultScreen, ScreenTrait};

pub struct Framework {
    pub current_screen: Box<dyn ScreenTrait>,
}

impl Framework {
    pub fn new() -> Self {
        Framework {
            current_screen: Box::new(DefaultScreen {}),
        }
    }
}