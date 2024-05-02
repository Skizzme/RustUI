mod renderer;
mod shader;
// mod gl20;
mod screen;
mod animation;
mod font;
mod gl20;

use std::cmp::max;
use std::thread;
use std::time::{Duration, Instant};
use glfw;
use glfw::{Context, fail_on_errors, PWindow, SwapInterval, WindowEvent, WindowHint};
use gl11::*;
use gl11::types::*;
use gl;
use gl::MULTISAMPLE;
use winapi::um::wincon::FreeConsole;
use crate::renderer::Renderer;
use crate::screen::{GuiScreen};

const WIDTH: u32 = 1920/2;
const HEIGHT: u32 = 1080/2;
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
pub struct WindowM {
    pub screen_width: u32,
    pub screen_height: u32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub frame_delta: f64,
    pub renderer: Renderer,
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if !(args.len() > 1 && args[1] == "console") {
        unsafe {
            FreeConsole();
        }
    }
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::Resizable(false));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Any));
    glfw.window_hint(WindowHint::Samples(Some(8u32)));

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed).expect("Failed to make window");

    window.make_current();
    window.set_all_polling(true);

    unsafe {
        gl11::load_with(|f_name| {
            glfw.get_proc_address_raw(f_name)
        });
        gl::load_with(|f_name| {
            glfw.get_proc_address_raw(f_name)
        });

        let renderer = renderer::Renderer::new();

        let mut window_m =
            WindowM {
                screen_width: WIDTH,
                screen_height: HEIGHT,
                mouse_x: 0.0,
                mouse_y: 0.0,
                frame_delta: 0.0,
                renderer,
            };
        let mut current_screen = screen::MainScreen::new();

        let mut last_frame = Instant::now();
        let b = Instant::now();
        let ft = font::Font::new("src\\resources\\fonts\\ProductSans.ttf", 32f32, &window_m.renderer);
        println!("Font took {:?} to load...", b.elapsed());
        while !window.should_close() {

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::CursorPos(x, y) => {
                        window_m.mouse_x = x as f32;
                        window_m.mouse_y = y as f32;
                    }
                    WindowEvent::Key(key, code, action, mods) => {
                        current_screen.key_press(key, code, action, mods)
                    }
                    _ => {}
                }
            }

            pre_render(&mut window);

            if !window.is_focused() {
                glfw.set_swap_interval(SwapInterval::Sync(0));
                let target_delta = (1.0/BACKGROUND_FPS);
                thread::sleep(Duration::from_secs_f32(target_delta));
            } else {
                glfw.set_swap_interval(SwapInterval::Sync(1));
            }
            window_m.frame_delta = last_frame.elapsed().as_secs_f64();
            last_frame = Instant::now();

            ft.draw_string("Comfortaa-Light", 10.0, 10.0, 0xffffffff);
            current_screen.draw(&window_m);

            post_render(&mut window);
        }
    }
}

unsafe fn pre_render(window: &mut PWindow,) {
    Viewport(0, 0, WIDTH as GLsizei, HEIGHT as GLsizei);

    Clear(DEPTH_BUFFER_BIT);
    MatrixMode(PROJECTION);
    LoadIdentity();
    Ortho(0 as GLdouble, WIDTH as GLdouble, HEIGHT as  GLdouble, 0 as GLdouble, 1000 as GLdouble, 3000 as GLdouble);
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