use std::rc::Rc;
use std::sync::Mutex;

use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};

use crate::components::elements::{Drawable, KeyboardInput, MouseInput};
use crate::components::window::Window;

pub trait ScreenTrait {
    // TODO: Maybe make some sort of pre-render process, allowing for off-screen rendering of framebuffers / effects like blur etc
    unsafe fn draw(&mut self, window: &mut Window);
    fn key_press(&mut self, key: Key, code: Scancode, action: Action, mods: Modifiers);
    fn event(&mut self, event: WindowEvent, window: &Window);
    fn elements(&self) -> Vec<Element>;
}

// #[derive(Clone)]
pub enum Element<'a> {
    Drawable(&'a mut dyn Drawable),
    KeyboardReceiver(&'a mut dyn KeyboardInput),
    MouseInputs(&'a mut dyn MouseInput),
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