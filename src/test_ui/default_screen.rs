use std::fs::read_to_string;
use std::time::Instant;

use gl::RGBA;
// use gl::{GenTextures, TexImage2D, UNSIGNED_BYTE};
// use gl::types::{GLdouble, GLint};
use glfw::{Action, Key, Modifiers, Scancode, WindowEvent};
use glfw::Action::Press;
use image::open;

use crate::components::render::animation::Animation;
use crate::components::render::font::ScaleMode;
use crate::components::render::shader::Shader;
use crate::components::render::texture::Texture;
use crate::components::screen::GuiScreen;
use crate::components::window::Window;

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
}

impl DefaultScreen {
    pub unsafe fn new() -> Self {
        DefaultScreen {
            move_progressive: Animation::new(),
            move_log: Animation::new(),
            move_cubic: Animation::new(),
            target: 200f64,
            circ_shader: Shader::new(read_to_string("src\\resources\\shaders\\spin_circle\\vertex.glsl").unwrap(), read_to_string("src\\resources\\shaders\\spin_circle\\fragment.glsl").unwrap()),
            init: Instant::now(),
            tex: None,
            offset_x: 0.0,
            offset_y: 0.0,
            dragging: (false, 0.0, 0.0, 0.0, 0.0),
            scroll: 50.0,
        }
    }
}

impl GuiScreen for DefaultScreen {
    unsafe fn draw(&mut self, m: &mut Window) {
        if self.tex.is_none() {
            let img = open("C:\\Users\\farre\\Pictures\\an event about to occur.png").unwrap().into_rgba8();
            self.tex = Some(Texture::create(m.renderer.clone(), img.width() as i32, img.height() as i32, &img.into_raw(), RGBA));
        }
        if self.dragging.0 {
            self.offset_x = self.dragging.3 + (m.mouse_x as f32 - self.dragging.1);
            self.offset_y = self.dragging.4 +(m.mouse_y as f32 - self.dragging.2);
        }

        // self.move_progressive.animate(self.scroll as f64, 1.5f64, AnimationType::Progressive(10f64), m);
        // self.tex.as_mut().unwrap().draw();
        // self.tex.as_ref().unwrap().render();
        // self.move_cubic.animate(m.mouse_x as f64, 1f64, AnimationType::CubicIn, m);
        // let font = m.fonts.get_font("JetBrainsMono-Medium").set_wrapping(Wrapping::None);
        // m.renderer.draw_rounded_rect(self.move_progressive.get_value() as f32, 10.0, self.move_progressive.get_value() as f32 + 200.0, 10.0 + 100.0, 10.0, 0xff909090);
        //(self.move_progressive.get_value() / 10.0) as f32
        // self.tex.as_mut().unwrap().render();
        // self.tex.as_mut().unwrap().bind();
        // m.renderer.draw_texture_rect(0.0, 0.0, 1200.0, 1200.0, 0xff909090);
        // self.tex.as_mut().unwrap().unbind();
        // m.renderer.draw_rounded_rect(self.move_cubic.get_value() as f32, 230.0, self.move_cubic.get_value() as f32 + 200.0, 330.0, 10.0, 0xff909090);
        // TODO: Make some sort of text element method that does not use gl immediate drawing, and instead it would create a VBO etc with all the chars and such
        m.fonts.get_font("JetBrainsMono-Medium", false).scale_mode(ScaleMode::Quality).draw_string(self.scroll, "test_ui", self.offset_x, self.offset_y, 0xff00ff90);
        // Enable(BLEND);
        // Enable(TEXTURE_2D);
        // self.circ_shader.bind();
        // self.circ_shader.put_float("u_size", vec![100.0, 100.0]);
        // self.circ_shader.put_float("u_radius", vec![5.0]);
        // self.circ_shader.put_float("u_color", m.renderer.get_rgb(0xffffffff));
        // self.circ_shader.put_float("u_time", vec![self.init.elapsed().as_secs_f32()]);
        // println!("{}", self.init.elapsed().as_secs_f32());
        //
        // m.renderer.draw_texture_rect(100.0, 100.0, 300.0, 300.0, 0xffffffff);
        // self.circ_shader.unbind();
    }

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
            WindowEvent::MouseButton(button, action, mods) => {
                self.dragging = (action == Press, window.mouse_x, window.mouse_y, self.offset_x, self.offset_y);
            }
            WindowEvent::CursorEnter(_) => {}
            WindowEvent::Scroll(x, y) => {
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