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

use std::{fs, ptr};
use std::os::raw::c_int;
use std::time::Instant;
use gl::{ActiveTexture, ARRAY_BUFFER, BindBuffer, BindVertexArray, BufferData, DrawElements, ELEMENT_ARRAY_BUFFER, GenBuffers, GenVertexArrays, STATIC_DRAW, TRIANGLES, UNSIGNED_INT, VertexArrayElementBuffer, VertexArrayVertexBuffer};
use gl::types::GLsizeiptr;
use glfw::{Action, Cursor, ffi, MouseButton, StandardCursor, WindowHint, WindowMode};
use image::open;
use winapi::um::wincon::FreeConsole;
use rand::{random, Rng, thread_rng};
use RustUI::components::bounds::Bounds;

use RustUI::components::context::{context, ContextBuilder, UIContext};
use RustUI::components::framework::element::{Element, ElementBuilder};
use RustUI::components::framework::event::Event;
use RustUI::components::framework::screen::ScreenTrait;
use RustUI::components::render::font::FontRenderer;
use RustUI::components::wrapper::texture::Texture;
use RustUI::gl_binds::gl11::{BLEND, EnableClientState, Finish, FLOAT, RGBA, TexCoordPointer, TEXTURE_COORD_ARRAY, VERTEX_ARRAY, VertexPointer};
use RustUI::gl_binds::gl20::{EnableVertexAttribArray, FALSE, TEXTURE0, TEXTURE_2D, TEXTURE_COORD_ARRAY_BUFFER_BINDING, VertexAttribPointer};
use RustUI::gl_binds::gl30::Enable;

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
    fr: FontRenderer,
}

impl TestScreen {
    pub unsafe fn new() -> Self {
        let mut t = fs::read_to_string("test.js").unwrap();
        TestScreen {
            text: t,
            fr: context().fonts().renderer("main"),
        }
    }
}

impl ScreenTrait for TestScreen {
    unsafe fn handle(&mut self, event: &Event) {
        match event {
            Event::Render(_) => {
                self.fr.draw_string(30.0, "something", (0.0, 100.0), 0xffffffff);
                context().tex.render();
                // let st = Instant::now();
                self.fr.draw_string_inst(30.0, format!("{:?}", context().fps()), (200.0, 100.0), 0xffffffff);
                // Finish();
                // println!("Instanced: {:?}", st.elapsed());
                // let st = Instant::now();
                context().fonts().renderer("main").draw_string_inst(30.0, &self.text, (200.0, 300.0), 0xffffffff);
                // println!("Direct: {:?}", st.elapsed());
            }
            Event::MouseClick(_, _) => {}
            Event::Keyboard(_, _, _) => {}
            _ => {}
        }
    }

    unsafe fn init(&mut self) -> Vec<Element> {
        let mut el_1 = ElementBuilder::new();

        el_1.bounds(Bounds::xywh(5.0, 100.0, 100.0, 100.0));
        el_1.draggable(true);
        el_1.handler(|el, event| {
            match event {
                Event::Render(_) => {
                    let mouse = context().window().mouse();
                    let (width, height) = context().fonts().renderer("main").draw_string_inst(40.0, format!("{:?}", mouse.pos()), el.bounds(), 0xffffffff);
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
                    // context().renderer().draw_rect(*el.bounds(), 0xff90ff20);z
                    let hovering = el.hovering();
                    el.bounds().draw_bounds(if hovering { 0xff10ff10 } else { 0xffffffff });
                },
                Event::MouseClick(button, action) => {
                    if el.hovering() && *action == Action::Press {
                        let v = !context().p_window().uses_raw_mouse_motion();
                        println!("change {}", v);
                        context().p_window().set_raw_mouse_motion(v);
                        // let v = !context().p_window().is_decorated();
                        // context().p_window().set_decorated(v);
                    }
                }
                _ => {}
            }
        });

        el_1.child(el_1_c.build());

        vec![el_1.build()]
    }
}