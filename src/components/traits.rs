use glfw::WindowEvent;

use crate::components::events::{KeyboardEvent, MouseEvent};
use crate::components::window::Window;

struct Object {
    position: (f32, f32),
    dims: (f32, f32),
}

trait Drawable {
    /// Returns the updated object, if it has been, with the newest position and dimensions.
    fn draw<'a>(&self, window: &mut Window) -> &'a Object;

    fn object<'a>() -> &'a Object;
}

trait MouseInput {
    fn mouse_button(&self, window: &mut Window, action: MouseEvent);

    fn object<'a>() -> &'a Object;
}

trait KeyboardInput {
    fn key_action(&self, window: &mut Window, action: KeyboardEvent);

    fn focused(&self) -> bool;

    /// Returns the element to be focused when `Tab` is pressed
    fn next(&self) -> Self;

    /// Returns the element to be focused when `Shift+Tab` is pressed
    fn previous(&self) -> Self;

    fn object<'a>() -> &'a Object;
}