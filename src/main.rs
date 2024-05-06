mod renderer;
mod shader;
mod screen;
mod animation;
mod font;
mod gl20;
mod default_screen;
mod window;
mod texture;

use std::fs::{read, read_to_string};
use std::mem::size_of_val;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use glfw;
use glfw::{Context, fail_on_errors, Glfw, GlfwReceiver, PWindow, SwapInterval, WindowEvent, WindowHint};
use gl11::*;
use gl11::types::*;
use gl;
use gl::{ARRAY_BUFFER, BindBuffer, BindVertexArray, BufferData, GenBuffers, GenVertexArrays, MULTISAMPLE, STATIC_DRAW};
use image::{EncodableLayout, open};
use winapi::um::wincon::FreeConsole;
use crate::default_screen::DefaultScreen;
use crate::font::FontManager;
use crate::renderer::Renderer;
use crate::screen::{GuiScreen};
use crate::shader::Shader;
use crate::texture::Texture;
use crate::window::Window;

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
        let mut frames = 0;
        let mut last_fps = Instant::now();

        // let shader = Shader::new(read_to_string("src\\resources\\shaders\\test\\vertex.glsl").unwrap(),
        //                                  read_to_string("src\\resources\\shaders\\test\\fragment.glsl").unwrap());
        //
        // let mut vao = 0;
        // let mut vbo = 0;
        // GenVertexArrays(1, &mut vao);
        // BindVertexArray(vao);
        //
        // let vertices: [[f32; 3]; 4] =
        //     [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 10.0, 0.0], [0.0, 10.0, 0.0]];
        //
        // GenBuffers(1, &mut vbo);
        // BindBuffer(ARRAY_BUFFER, vbo);
        // BufferData(
        //     ARRAY_BUFFER,
        //     size_of_val(&vertices) as isize,
        //     vertices.as_ptr().cast(),
        //     STATIC_DRAW
        // );
        // let data = read("C:\\Users\\farre\\Pictures\\alpha_album_cover_high_purp.jpg").unwrap();
        while !window.p_window.should_close() {
            // shader.bind();
            // VertexPointer()
            let st = Instant::now();
            window.run(Box::new(&mut current_screen), last_frame);
            last_frame = st;
            if last_fps.elapsed().as_secs_f32() > 1.0 {
                println!("FPS {:?}", frames);
                last_fps = Instant::now();
                frames = 0;
            }
            frames += 1;
        }
    }
}