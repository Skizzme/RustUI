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

use glfw::{Action, MouseButton, WindowHint, WindowMode};
use winapi::um::wincon::FreeConsole;
use RustUI::components::bounds::Bounds;

use RustUI::components::context::{context, ContextBuilder, UIContext};
use RustUI::components::framework::element::{Element, ElementBuilder};
use RustUI::components::framework::event::Event;
use RustUI::components::framework::screen::ScreenTrait;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if !(args.len() > 1 && args[1] == "console") {
        unsafe {
            FreeConsole();
        }
    }

    unsafe {
        let mut builder = ContextBuilder::new();
        builder.title("Test");
        builder.dims(1920/2, 1080/2);
        builder.hint(WindowHint::Resizable(false));
        builder.build();

        context().framework().set_screen(TestScreen::new());
        context().do_loop()
    }
}


pub struct TestScreen {
    pub text: String,
}

impl TestScreen {
    pub fn new() -> Self {
        TestScreen {
            text: "SomeTExt".to_string(),
        }
    }
}

impl ScreenTrait for TestScreen {
    unsafe fn event(&mut self, event: &Event) {
        match event {
            Event::Render(_) => {
                context().fonts().get_font("main").draw_string(30.0, &self.text, (200.0, 100.0), 0xffffffff);
                context().fonts().get_font("main").draw_string(30.0, &self.text, (200.0, 300.0), 0xffffffff);
            }
            Event::MouseClick(_, _) => {}
            Event::Keyboard(_, _, _) => {}
            _ => {}
        }
    }

    unsafe fn register_elements(&mut self) -> Vec<Element> {
        let mut el_1 = ElementBuilder::new();

        el_1.bounds(Bounds::xywh(5.0, 100.0, 100.0, 100.0));
        el_1.draggable(true);
        el_1.handler(|el, event| {
            match event {
                Event::Render(_) => {
                    let mouse = context().window().mouse();
                    let (width, height) = context().fonts().get_font("main").draw_string(40.0, format!("{:?}", mouse.pos()), el.bounds(), 0xffffffff);
                    el.bounds().set_width(width);
                    el.bounds().set_height(height);
                    let hovering = el.hovering();
                    el.bounds().draw_bounds(if hovering { 0xff10ff10 } else { 0xffffffff });
                }
                _ => {}
            }
        });

        let mut el_1_c = ElementBuilder::new();
        el_1_c.bounds(Bounds::xywh(0.0, 0.0, 10.0, 10.0));
        el_1_c.draggable(false);
        el_1_c.handler(|el, event| {
            match event {
                Event::Render(_) => {
                    // context().renderer().draw_rect(*el.bounds(), 0xff90ff20);
                    let hovering = el.hovering();
                    el.bounds().draw_bounds(if hovering { 0xff10ff10 } else { 0xffffffff });
                },
                _ => {}
            }
        });

        el_1.child(el_1_c.build());

        vec![el_1.build()]
    }
}