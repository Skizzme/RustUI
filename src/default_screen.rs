use std::fs::read_to_string;
use std::time::Instant;
use gl::{GenTextures, TexImage2D, UNSIGNED_BYTE};
use gl::types::{GLdouble, GLint};
use glfw::{Action, Key, Modifiers, Scancode};
use image::{EncodableLayout, open};
use crate::animation::{Animation, AnimationType};
use crate::gl20::{PopMatrix, PushMatrix, RGBA, Scaled};
use crate::screen::GuiScreen;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::Window;

pub struct DefaultScreen {
    move_progressive: Animation,
    move_log: Animation,
    move_cubic: Animation,
    target: f64,
    circ_shader: Shader,
    init: Instant,
    tex: Option<Texture>,
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
        }
    }
}

impl GuiScreen for DefaultScreen {
    unsafe fn draw(&mut self, m: &mut Window) {
        if self.tex.is_none() {
            let img = open("C:\\Users\\farre\\Pictures\\an event about to occur.png").unwrap().into_rgba8();
            self.tex = Some(Texture::create(m.renderer.clone(), img.width() as i32, img.height() as i32, img.into_raw(), RGBA));
        }
        self.move_progressive.animate(m.mouse_x as f64, 0.4f64, AnimationType::Progressive(10f64), m);
        self.move_cubic.animate(m.mouse_x as f64, 1f64, AnimationType::CubicIn, m);
        // m.renderer.draw_rounded_rect(self.move_progressive.get_value() as f32, 10.0, self.move_progressive.get_value() as f32 + 200.0, 10.0 + 100.0, 10.0, 0xff909090);
        //(self.move_progressive.get_value() / 10.0) as f32
        m.fonts.get_font("JetBrainsMono-Medium").draw_string(20.0, read_to_string("test.js").unwrap().as_str(), 5.0, 5.0, 0xffaaaaaa);
        self.tex.as_mut().unwrap().render();
        // self.tex.as_mut().unwrap().bind();
        // m.renderer.draw_texture_rect(0.0, 0.0, 1200.0, 1200.0, 0xff909090);
        // self.tex.as_mut().unwrap().unbind();
        // m.renderer.draw_rounded_rect(self.move_cubic.get_value() as f32, 230.0, self.move_cubic.get_value() as f32 + 200.0, 330.0, 10.0, 0xff909090);

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
}