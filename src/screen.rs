use std::time::{Duration, Instant};
use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};
use crate::animation::{AnimationType, Animation};
use crate::renderer::Renderer;
use crate::Window;

pub trait GuiScreen {

    unsafe fn draw(&mut self, window: &mut Window);

    fn key_press(&mut self, key: Key, code: Scancode, action: Action, mods: Modifiers);
    fn event(&mut self, event: WindowEvent, window: &Window);
}