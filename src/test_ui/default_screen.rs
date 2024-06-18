use std::time::Instant;

use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};
use glfw::Action::Press;

use crate::asset_manager;
use crate::components::elements::Drawable;
use crate::components::render::animation::Animation;
use crate::components::render::bounds::Bounds;
use crate::components::screen::GuiScreen;
use crate::components::window::Window;
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::test_ui::test_object::DrawThing;

#[allow(unused)]
pub struct DefaultScreen {
    move_progressive: Animation,
    move_log: Animation,
    move_cubic: Animation,
    target: f64,
    circ_shader: Shader,
    init: Instant,
    tex: Option<Texture>,
    offset_x: f32,
    offset_y: f32,
    dragging: (bool, f32, f32, f32, f32),
    scroll: f32,
    test_draw: DrawThing,
}

impl DefaultScreen {
    pub unsafe fn new(window: &mut Window) -> Self {
        DefaultScreen {
            move_progressive: Animation::new(),
            move_log: Animation::new(),
            move_cubic: Animation::new(),
            target: 200f64,
            circ_shader: Shader::new(asset_manager::file_contents_str ("shaders\\spin_circle\\vertex.glsl").unwrap(), asset_manager::file_contents_str ("shaders\\spin_circle\\fragment.glsl").unwrap()),
            init: Instant::now(),
            tex: None,
            offset_x: 0.0,
            offset_y: 0.0,
            dragging: (false, 0.0, 0.0, 0.0, 0.0),
            scroll: 50.0,
            test_draw: DrawThing::new(Bounds::from_xywh(10.0, 10.0, 200.0, 100.0), window),
        }
    }
}

impl GuiScreen for DefaultScreen {
    unsafe fn draw(&mut self, m: &mut Window) {
        if self.dragging.0 {
            self.offset_x = self.dragging.3 + (m.mouse_x as f32 - self.dragging.1);
            self.offset_y = self.dragging.4 +(m.mouse_y as f32 - self.dragging.2);
        }
        // TODO: Make some sort of text element method that does not use gl immediate drawing, and instead it would create a VBO etc with all the chars and such
        self.test_draw.draw(m);
    }
    #[allow(unused)]
    fn key_press(&mut self, key: Key, code: Scancode, action: Action, mods: Modifiers) {
        match action {
            Action::Release => {}
            Action::Press => {
                println!("press");
                if self.target == 200f64 {
                    self.target = 400f64;
                } else {
                    self.target = 200f64;
                }}
            Action::Repeat => {}
        }
    }

    fn event(&mut self, event: WindowEvent, window: &Window) {
        match event {
            WindowEvent::Pos(_, _) => {}
            WindowEvent::Size(_, _) => {}
            WindowEvent::Close => {}
            WindowEvent::Refresh => {}
            WindowEvent::Focus(_) => {}
            WindowEvent::Iconify(_) => {}
            WindowEvent::FramebufferSize(_, _) => {}
            WindowEvent::MouseButton(_button, action, _mods) => {
                self.dragging = (action == Press, window.mouse_x, window.mouse_y, self.offset_x, self.offset_y);
            }
            WindowEvent::CursorEnter(_) => {}
            WindowEvent::Scroll(_x, y) => {
                self.scroll += y as f32;
            }
            WindowEvent::Key(_, _, _, _) => {}
            WindowEvent::Char(_) => {}
            WindowEvent::CharModifiers(_, _) => {}
            WindowEvent::FileDrop(_) => {}
            WindowEvent::Maximize(_) => {}
            WindowEvent::ContentScale(_, _) => {}
            _ => {}
        }
    }
}