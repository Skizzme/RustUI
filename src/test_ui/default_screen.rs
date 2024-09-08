use std::fs::read;
use std::path;
use std::rc::Rc;
use std::sync::Mutex;
use std::time::{Instant, UNIX_EPOCH};

use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};

use crate::asset_manager::file_contents_str;
use crate::components::elements::Drawable;
use crate::components::position::Pos;
use crate::components::render::animation::{Animation, AnimationType};
use crate::components::render::bounds::Bounds;
use crate::components::render::color::Color;
use crate::components::render::mask::FramebufferMask;
use crate::components::screen::{Element, Screen, ScreenTrait};
use crate::components::window::Window;
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::gl30::{BLEND, Enable};
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
    // test_draw: DrawThing,
    mask: FramebufferMask,
    screen: Screen,
    dummy: &'a str,
}

impl<'a> TestScreen<'a> {
    pub unsafe fn new(window: &mut Window) -> Self {
        window.fonts.set_font_bytes("ProductSans", read("src/assets/fonts/ProductSans.ttf".replace("/", path::MAIN_SEPARATOR_STR)).unwrap()).load_font("ProductSans", false);
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
            mask: FramebufferMask::new(window),
            screen: Screen::new(),
            dummy: "",
        };
        ds.screen.add_element(Element::Drawable(Box::new(DrawThing::new(Bounds::ltrb(0.0, 0.0, 100.0, 100.0), window))));
        ds
    }
}

impl<'a> ScreenTrait for TestScreen<'a> {
    unsafe fn draw(&mut self, w: &mut Window) {
        if self.dragging.0 {
            self.offset_x = self.dragging.3 + (w.mouse_x - self.dragging.1);
            self.offset_y = self.dragging.4 +(w.mouse_y - self.dragging.2);
        }

        let b = Bounds::xywh(20.0, 20.0, 150.0, 150.0);
        // w.renderer.draw_rect(b, Color::from_hsv((UNIX_EPOCH.elapsed().unwrap().as_secs_f64() % 5.0 / 5.0) as f32, 1.0, 1.0));
        // w.renderer.draw_rounded_rect(b, 20.0, 0xffffffff);
        w.renderer.draw_rect(Bounds::xywh(0.0, 0.0, w.width as f32, w.height as f32), 0xff100000);

        w.fonts.get_font("ProductSans").unwrap().draw_string((self.move_progressive.value() * 15.0 + 20.0) as f32, "TestABCjskIlG", 4.0, 4.0, Color::from_u32(0xffffffff));

        if self.move_log.value() > 0.99 && self.move_log.target() == 1.0 {
            self.move_log.set_target(0.0);
        } else if self.move_log.value() < 0.01 {
            self.move_log.set_target(1.0);
        }

        if w.mouse_x > 100.0 && w.mouse_y > 100.0 {
            self.move_progressive.animate_target(1.0, 1.0, AnimationType::Sin, w);
        } else {
            self.move_progressive.animate_target(0.0, 1.0, AnimationType::Sin, w);
        }
    }

    fn base(&mut self) -> &mut Screen {
        &mut self.screen
    }
}