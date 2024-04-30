mod renderer;
mod shader;
mod gl20;


use std::thread;
use std::time::{Duration, Instant};
use glfw;
use glfw::{Context, fail_on_errors, PWindow, WindowEvent, WindowHint};
use gl11::*;
use gl11::types::*;

const WIDTH: u32 = 1920/2;
const HEIGHT: u32 = 1080/2;
const TITLE: &str = "Test";
const FPS: f32 = 144f32;

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

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(2, 0));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Any));

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed).expect("Failed to make window");

    window.make_current();
    window.set_all_polling(true);

    let (mut mouseX, mut mouseY) = (0f32, 0f32);
    unsafe {
        gl11::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        // gl::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        gl20::load_with(|f_name| glfw.get_proc_address_raw(f_name));

        let renderer = renderer::Renderer::new();

        while !window.should_close() {

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::CursorPos(x, y) => {
                        mouseX = x as f32;
                        mouseY = y as f32;
                    }
                    _ => {}
                }
            }

            pre_render(&mut window);

            renderer.draw_rounded_rect(10.0, 10.0, 100.0, 100.0, 15.0, 0xff909090);
            renderer.draw_rounded_rect(mouseX, mouseY, mouseX+200.0, mouseY+200.0, 25.0, 0xff909090);

            post_render(&mut window);

            thread::sleep(Duration::from_secs_f32(1f32/FPS))
        }
    }
}

unsafe fn pre_render(window: &mut PWindow) {
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
}

unsafe fn post_render(window: &mut PWindow) {
    let err = GetError();
    if err != 0 {
        println!("OpenGL: {:?}", err);

    }
    window.swap_buffers();
}