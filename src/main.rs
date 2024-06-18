#![allow(non_snake_case)]
// GENERATE OPEN GL BINDINGS FOR ANY VERSION
// extern crate gl_generator;
// use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
// use std::env;
// use std::fs::File;
// use std::path::Path;
//
// fn main() {
//     let mut file = File::create("src/gl.rs").unwrap();
//
//     Registry::new(Api::Gl, (3, 0), Profile::Core, Fallbacks::All, [])
//         .write_bindings(GlobalGenerator, &mut file)
//         .unwrap();
// }

use std::time::Instant;

use glfw::WindowMode;
use winapi::um::wincon::FreeConsole;

use RustUI::components::window::Window;
use RustUI::test_ui::default_screen::DefaultScreen;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if !(args.len() > 1 && args[1] == "console") {
        unsafe {
            FreeConsole();
        }
    }

    unsafe {
        let mut window = Window::create("Test", 1920/2, 1080/2, "src/assets/fonts/", "", Vec::new(), WindowMode::Windowed, 30);
        let mut current_screen = DefaultScreen::new(&mut window);
        let mut last_frame = Instant::now();
        let mut frames = 0;
        let mut last_fps = Instant::now();

        while !window.p_window.should_close() {
            window.frame(Box::new(&mut current_screen), last_frame);
            last_frame = Instant::now();
            if last_fps.elapsed().as_secs_f32() > 1.0 {
                println!("FPS {:?}", frames);
                last_fps = Instant::now();
                frames = 0;
            }
            frames += 1;
        }
    }
}