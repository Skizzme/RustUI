use std::path;
use std::rc::Rc;
use std::sync::Mutex;
use std::time::{Instant, UNIX_EPOCH};

use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};

use crate::asset_manager::file_contents_str;
use crate::components::elements::Drawable;
use crate::components::render::animation::{Animation, AnimationType};
use crate::components::render::bounds::Bounds;
use crate::components::render::color::Color;
use crate::components::render::mask::FramebufferMask;
use crate::components::screen::{Element, ScreenTrait};
use crate::components::window::Window;
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::test_ui::test_object::DrawThing;

#[allow(unused)]
pub struct TestScreen<'a> {
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
    test_draw: Rc<Mutex<DrawThing>>,
    mask: FramebufferMask,
    elements: Vec<Element>,
    dummy: &'a str,
}

impl<'a> TestScreen<'a> {
    pub unsafe fn new(window: &mut Window) -> Self {
        let mut ds = TestScreen {
            move_progressive: Animation::new(),
            move_log: Animation::new(),
            move_cubic: Animation::new(),
            target: 200f64,
            circ_shader: Shader::new(file_contents_str("shaders/spin_circle/vertex.glsl".replace("/", path::MAIN_SEPARATOR_STR)).unwrap(), file_contents_str("shaders/spin_circle/fragment.glsl".replace("/", path::MAIN_SEPARATOR_STR)).unwrap()),
            init: Instant::now(),
            tex: None,
            offset_x: 0.0,
            offset_y: 0.0,
            dragging: (false, 0.0, 0.0, 0.0, 0.0),
            scroll: 50.0,
            test_draw: Rc::new(Mutex::new(DrawThing::new(Bounds::from_xywh(10.0, 10.0, 200.0, 100.0), window))),
            mask: FramebufferMask::new(window),
            elements: vec![],
            dummy: "",
        };
        ds.elements.push(Element::Drawable(ds.test_draw.clone()));
        ds
    }
}

impl<'a> ScreenTrait<'a> for TestScreen<'a> {
    unsafe fn draw(&mut self, w: &mut Window) {
        if self.dragging.0 {
            self.offset_x = self.dragging.3 + (w.mouse_x - self.dragging.1);
            self.offset_y = self.dragging.4 +(w.mouse_y - self.dragging.2);
        }

        let b = Bounds::from_xywh(20.0, 20.0, 150.0, 150.0);
        // w.renderer.draw_rect(b, Color::from_hsv((UNIX_EPOCH.elapsed().unwrap().as_secs_f64() % 5.0 / 5.0) as f32, 1.0, 1.0));
        // w.renderer.draw_rounded_rect(b, 20.0, 0xffffffff);
        // w.renderer.draw_rect(Bounds::from_xywh(0.0, 0.0, w.width as f32, w.height as f32), 0xffffffff);

        if self.move_log.value() > 0.99 && self.move_log.target() == 1.0 {
            self.move_log.set_target(0.0);
        } else if self.move_log.value() < 0.01 {
            self.move_log.set_target(1.0);
        }

        self.move_log.animate(0.6, AnimationType::Progressive(10.0), w);

        self.mask.begin_mask();
        w.renderer.draw_circle(b.center_x(), b.center_y(), (self.move_log.value() * 150.0) as f32, 0xffffffff);
        self.mask.end_mask();
        self.mask.begin_draw();
        // TODO: Make some sort of text element method that does not use gl immediate drawing, and instead it would create a VBO etc with all the chars and such
        w.renderer.draw_rect(b, Color::from_hsv((UNIX_EPOCH.elapsed().unwrap().as_secs_f64() % 5.0 / 5.0) as f32, 0.6, 1.0).set_alpha_f32(0.9));
        self.mask.end_mask();
        self.mask.render(w);
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
                self.dragging = (action == Action::Press, window.mouse_x, window.mouse_y, self.offset_x, self.offset_y);
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

    fn elements(&self) -> Vec<Element> {
        self.elements.clone()
    }
}