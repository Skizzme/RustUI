use std::rc::Rc;
use crate::components::events::{KeyboardEvent, MouseEvent};
use crate::components::position::Pos;
use crate::components::render::bounds::Bounds;
use crate::components::window::Window;

// pub struct Drawable {
//     draw_fn: Box<dyn FnMut(&mut Window) -> Pos>,
//     pos: Pos,
// }
//
// impl Drawable {
//     pub fn new<F: FnMut(&mut Window) -> Pos + 'static>(pos: impl Into<Pos>, draw: F) -> Self {
//         let draw = Box::new(draw);
//         Drawable {
//             pos: pos.into(),
//             draw_fn: draw,
//         }
//     }
//
//     pub fn draw(&mut self, w: &mut Window) -> Pos {
//         (self.draw_fn)(w)
//     }
//
//     pub fn pos(&self) -> Pos {
//         self.pos.clone()
//     }
// }

pub trait Drawable {
    unsafe fn draw<'a>(&mut self, window: &mut Window);

    fn bounds(&self) -> &Bounds;
}

pub trait MouseInput {
    fn mouse_button(&mut self, window: &mut Window, action: &MouseEvent);

    fn bounds(&self) -> &Bounds;
}

pub trait KeyboardInput {
    fn key_action(&self, window: &mut Window, action: &KeyboardEvent);

    fn focused(&self) -> bool;
    fn set_focused(&self, value: bool);

    fn bounds(&self) -> &Bounds;
}

/// A trait for anything to implement that might have multiple elements
pub trait Container<F> {
    fn children() -> Vec<F>;
}