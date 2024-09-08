use std::rc::Rc;
use std::sync::Mutex;

use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};

use crate::components::elements::{Drawable, KeyboardInput, MouseInput};
use crate::components::events::Event;
use crate::components::position::Pos;
use crate::components::window::Window;

pub struct Screen {
    elements: Vec<Element>,
}

impl Screen {

    pub fn new() -> Self {
        Screen {
            elements: vec![],
        }
    }

    pub fn handle(&mut self, mut event: Event) {
        for mut element in &mut self.elements {
            element.handle(&mut event);
        }
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }

}

pub trait ScreenTrait {
    // TODO: Maybe make some sort of pre-render process, allowing for off-screen rendering of framebuffers / effects like blur etc
    unsafe fn draw(&mut self, window: &mut Window);
    fn base(&mut self) -> &mut Screen;
}

// #[derive(Clone)]
pub enum Element {
    Drawable(Box<dyn Drawable>),
    KeyboardReceiver(Box<dyn KeyboardInput>),
    MouseInputs(Box<dyn MouseInput>),
}

impl Element {
    pub fn handle(&mut self, event: &mut Event) {
        match event {
            Event::Mouse(ref mut w, event) => {
                match self {
                    Element::MouseInputs(e) => {
                        e.mouse_button(*w, event);
                    }
                    _ => {}
                }
            }
            Event::Keyboard(ref mut w, event) => {
                match self {
                    Element::KeyboardReceiver(e) => {
                        e.key_action(*w, &event);
                    }
                    _ => {}
                }
            }
            Event::Draw(ref mut window) => {
                match self {
                    Element::Drawable(e) => unsafe {
                        e.draw(*window)
                    }
                    _ => {}
                }
            }
        }
    }
}

// pub struct Elements<'a> {
//     pub drawables: Vec<&'a dyn Drawable>,
//     pub keyboard_inputs: Vec<&'a dyn KeyboardInput>,
//     pub mouse_inputs: Vec<&'a dyn MouseInput>,
// }
//
// impl<'a> Elements<'a> {
//     pub fn empty() -> Elements<'a> {
//         Elements {
//             drawables: Vec::new(),
//             keyboard_inputs: Vec::new(),
//             mouse_inputs: Vec::new(),
//         }
//     }
// }