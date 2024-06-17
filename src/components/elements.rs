use crate::components::events::{KeyboardEvent, MouseEvent};
use crate::components::render::bounds::Bounds;
use crate::components::window::Window;

pub trait Drawable {
    unsafe fn draw<'a>(&mut self, window: &mut Window);

    fn bounds(&self) -> &Bounds;
}

pub trait MouseInput {
    fn mouse_button(&mut self, window: &mut Window, action: MouseEvent);

    fn bounds(&self) -> &Bounds;
}

pub trait KeyboardInput {
    fn key_action(&mut self, window: &mut Window, action: KeyboardEvent);

    fn focused(&mut self) -> bool;
    fn set_focused(&mut self, value: bool);

    /// Returns the element to be focused when `Tab` is pressed
    fn next(&mut self) -> Self;

    /// Returns the element to be focused when `Shift+Tab` is pressed
    fn previous(&mut self) -> Self;

    fn bounds(&mut self) -> &Bounds;
}

/// A trait for anything to implement that might have multiple elements
pub trait Container<F> {
    fn children() -> Vec<F>;
}