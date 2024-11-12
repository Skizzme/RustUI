use glfw::WindowEvent;

use crate::components::bounds::Bounds;
use crate::components::wrapper::mouse::Mouse;

pub struct Window {
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) mouse: Mouse,
}

impl Window {
    pub fn new(width: i32, height: i32) -> Window {
        Window {
            width,
            height,
            mouse: Mouse::new(),
        }
    }

    pub fn handle(&mut self, event: &WindowEvent) {
        self.mouse.handle(event);
        match event {
            WindowEvent::Size(width, height ) => {
                self.width = *width;
                self.height = *height;
            }
            _ => {}
        }
        // println!("{:?}", event);
    }

    /// Creates a bounds object of `(0.0,0.0,width,height)`
    pub fn bounds(&self) -> Bounds {
        Bounds::xywh(0.0, 0.0, self.width as f32, self.height as f32)
    }

    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }
}