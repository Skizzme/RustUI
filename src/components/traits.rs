use glfw::WindowEvent;
use crate::components::window::Window;

trait Object {
    fn pos(&self) -> (f32, f32);
    fn dims(&self) -> (f32, f32);
}

trait Drawable {
    /// Returns width and height of this drawable thing
    fn draw(&self, window: &mut Window) -> (f32, f32);

    fn object() -> dyn Object;
}

trait MouseInput {
    fn mouse_button(&self, window: &mut Window, action: MouseEvent);

    fn object() -> dyn Object;
}

trait KeyboardInput {
    fn key_action(&self, window: &mut Window, action: WindowEvent::Key);

    fn focused(&self) -> bool;

    /// Returns the element to be focused when `Tab` is pressed
    fn next(&self) -> dyn KeyboardInput;

    /// Returns the element to be focused when `Shift+Tab` is pressed
    fn previous(&self) -> dyn KeyboardInput;

    fn object() -> dyn Object;
}