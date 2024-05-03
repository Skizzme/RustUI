mod renderer;
mod shader;
mod screen;
mod animation;
mod font;
mod gl20;
mod default_screen;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use glfw;
use glfw::{Context, fail_on_errors, Glfw, GlfwReceiver, PWindow, SwapInterval, WindowEvent, WindowHint};
use gl11::*;
use gl11::types::*;
use gl;
use gl::MULTISAMPLE;
use winapi::um::wincon::FreeConsole;
use crate::default_screen::DefaultScreen;
use crate::font::FontManager;
use crate::renderer::Renderer;
use crate::screen::{GuiScreen};

const TITLE: &str = "Test";
const FPS: f32 = 144f32;
const BACKGROUND_FPS: f32 = 30f32;

// extern crate gl_generator;

// GENERATE OPEN GL BINDINGS FOR ANY VERSION
// use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
// use std::env;
// use std::fs::File;
// use std::path::Path;

// fn main() {
//     let mut file = File::create("binding\\gl20").unwrap();
//
//     Registry::new(Api::Gl, (2, 0), Profile::Core, Fallbacks::All, [])
//         .write_bindings(GlobalGenerator, &mut file)
//         .unwrap();
// }

// Window metrics
pub struct Window {
    pub screen_width: u32,
    pub screen_height: u32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub frame_delta: f64,
    pub renderer: Rc<Renderer>,
    pub fonts: FontManager,

    // pub current_screen: Box<dyn GuiScreen>,

    p_window: PWindow,
    glfw: Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub unsafe fn create(width: u32, height: u32) -> Window {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(WindowHint::ContextVersion(4, 6));
        glfw.window_hint(WindowHint::Resizable(false));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Any));
        glfw.window_hint(WindowHint::Samples(Some(8u32)));

        let (mut p_window, events) = glfw.create_window(width, height, TITLE, glfw::WindowMode::Windowed).expect("Failed to make window");

        p_window.make_current();
        p_window.set_all_polling(true);

        gl11::load_with(|f_name| {
            glfw.get_proc_address_raw(f_name)
        });
        gl::load_with(|f_name| {
            glfw.get_proc_address_raw(f_name)
        });
        gl20::load_with(|f_name| {
            glfw.get_proc_address_raw(f_name)
        });

        let renderer = Rc::new(Renderer::new());

        Window {
            screen_width: width,
            screen_height: height,
            mouse_x: 0.0,
            mouse_y: 0.0,
            frame_delta: 0.0,
            renderer: renderer.clone(),
            fonts: FontManager::new(renderer.clone()),
            // current_screen: Box::new(default_screen::DefaultScreen::new()),
            p_window,
            glfw,
            events,
        }
    }

    pub unsafe fn run(&mut self, mut current_screen: Box<&mut dyn GuiScreen>, last_frame: Instant) {

        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::CursorPos(x, y) => {
                    self.mouse_x = x as f32;
                    self.mouse_y = y as f32;
                }
                WindowEvent::Key(key, code, action, mods) => current_screen.key_press(key, code, action, mods),
                _e => {}
            }
        }

        pre_render(self);

        if !self.p_window.is_focused() {
            self.glfw.set_swap_interval(SwapInterval::Sync(0));
            let target_delta = (1.0/BACKGROUND_FPS);
            thread::sleep(Duration::from_secs_f32(target_delta));
        } else {
            self.glfw.set_swap_interval(SwapInterval::Sync(1));
        }

        self.frame_delta = last_frame.elapsed().as_secs_f64();

        current_screen.draw(self);

        post_render(&mut self.p_window);
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if !(args.len() > 1 && args[1] == "console") {
        unsafe {
            FreeConsole();
        }
    }

    unsafe {
        let mut window = Window::create(1920/2, 1080/2);
        let mut current_screen = DefaultScreen::new();
        let mut last_frame = Instant::now();
        while !window.p_window.should_close() {
            let st = Instant::now();
            window.run(Box::new(&mut current_screen), last_frame);
            last_frame = st;
        }
    }
}

unsafe fn pre_render(window: &Window) {
    Viewport(0, 0, window.screen_width as GLsizei, window.screen_height as GLsizei);

    Clear(DEPTH_BUFFER_BIT);
    MatrixMode(PROJECTION);
    LoadIdentity();
    Ortho(0 as GLdouble, window.screen_width as GLdouble, window.screen_height as  GLdouble, 0 as GLdouble, 1000 as GLdouble, 3000 as GLdouble);
    Translated(0 as GLdouble, 0 as GLdouble, -2000 as GLdouble);

    Clear(COLOR_BUFFER_BIT);
    Enable(TEXTURE_2D);
    Enable(BLEND);
    BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    Enable(MULTISAMPLE);
}

unsafe fn post_render(window: &mut PWindow) {
    check_error();
    window.swap_buffers();
}

unsafe fn check_error() {
    let err = GetError();
    if err != 0 {
        println!("OpenGL: {:?}", err);
    }
}