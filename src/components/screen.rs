use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};

use crate::components::window::Window;

pub trait GuiScreen {
    // TODO: Maybe make some sort of pre-render process, allowing for off-screen rendering of framebuffers / effects like blur etc
    unsafe fn draw(&mut self, window: &mut Window);
    fn key_press(&mut self, key: Key, code: Scancode, action: Action, mods: Modifiers);
    fn event(&mut self, event: WindowEvent, window: &Window);
}