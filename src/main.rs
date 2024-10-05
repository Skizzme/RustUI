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

use std::ptr;
use std::time::Instant;
use gl::{ActiveTexture, ARRAY_BUFFER, BindBuffer, BindVertexArray, BufferData, DrawElements, ELEMENT_ARRAY_BUFFER, GenBuffers, GenVertexArrays, STATIC_DRAW, TRIANGLES, UNSIGNED_INT, VertexArrayElementBuffer, VertexArrayVertexBuffer};
use gl::types::GLsizeiptr;
use glfw::{Action, MouseButton, WindowHint, WindowMode};
use image::open;
use winapi::um::wincon::FreeConsole;
use RustUI::components::bounds::Bounds;

use RustUI::components::context::{context, ContextBuilder, UIContext};
use RustUI::components::framework::element::{Element, ElementBuilder};
use RustUI::components::framework::event::Event;
use RustUI::components::framework::screen::ScreenTrait;
use RustUI::components::wrapper::texture::Texture;
use RustUI::gl_binds::gl11::{BLEND, EnableClientState, FLOAT, RGBA, TexCoordPointer, TEXTURE_COORD_ARRAY, VERTEX_ARRAY, VertexPointer};
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
    tex: Texture,
}

impl TestScreen {
    pub unsafe fn new() -> Self {
        let img = open("C:\\Users\\farre\\Pictures\\an event about to occur.png").unwrap().into_rgba8();
        TestScreen {
            text: "SomeTExt".to_string(),
            tex: Texture::create(img.width() as i32, img.height() as i32, &img.into_raw(), RGBA),
        }
    }
}

impl ScreenTrait for TestScreen {
    unsafe fn event(&mut self, event: &Event) {
        match event {
            Event::Render(_) => {
                if let Some(f) = context().fonts().font("main") {
                    let st = Instant::now();
                    // self.tex.render();
                    self.tex.bind();
                    context().renderer().draw_texture_rect_uv(Bounds::ltrb(0.0, 0.0, 50.0, 50.0), Bounds::ltrb(0.0, 0.0, 1.0, 1.0), 0xffffffff);
                    self.tex.unbind();
                    println!("{:?}", st.elapsed());

                    // Enable(BLEND);
                    // Enable(TEXTURE_2D);
                    //
                    // let mut vao = 0;
                    // let mut vbo = 0;
                    // let mut uvo = 0;
                    // let mut ebo = 0;
                    // GenVertexArrays(1, &mut vao);
                    // GenBuffers(1, &mut vbo);
                    // GenBuffers(1, &mut uvo);
                    // GenBuffers(1, &mut ebo);
                    // BindVertexArray(vao);
                    //
                    // // Make buffers
                    // // let vertices = [[-0.5f32, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5],];
                    // let vertices = [[0.0, 50 as f32], [50 as f32, 50 as f32], [50 as f32, 0.0], [0.0, 0.0],];
                    //
                    // BindBuffer(ARRAY_BUFFER, vbo);
                    // BufferData(
                    //     ARRAY_BUFFER,
                    //     size_of_val(vertices.as_slice()) as GLsizeiptr,
                    //     vertices.as_ptr() as *const _,
                    //     STATIC_DRAW
                    // );
                    // EnableClientState(VERTEX_ARRAY);
                    // VertexPointer(2, FLOAT, 4 * 2, ptr::null());
                    //
                    // EnableClientState(TEXTURE_COORD_ARRAY);
                    // let uvs: [[f32; 2]; 4] = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
                    // BindBuffer(ARRAY_BUFFER, uvo);
                    // BufferData(
                    //     ARRAY_BUFFER,
                    //     size_of_val(uvs.as_slice()) as GLsizeiptr,
                    //     uvs.as_ptr() as *const _,
                    //     STATIC_DRAW
                    // );
                    // TexCoordPointer(2, FLOAT, 4 * 2, ptr::null());
                    //
                    // let indices = [0, 1, 2, 0, 2, 3];
                    // BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
                    // BufferData(
                    //     ELEMENT_ARRAY_BUFFER,
                    //     size_of_val(&indices) as isize,
                    //     indices.as_ptr().cast(),
                    //     STATIC_DRAW
                    // );

                    // context().renderer().texture_shader.bind();
                    // ActiveTexture(TEXTURE0);
                    // t.bind();
                    // BindVertexArray(vao);
                    // DrawElements(TRIANGLES, 6, UNSIGNED_INT, ptr::null());
                    // BindVertexArray(0);
                    // t.unbind();
                    // context().renderer().texture_shader.unbind();
                    // BindBuffer(ELEMENT_ARRAY_BUFFER, 0);
                    // BindBuffer(ARRAY_BUFFER, 0);
                }
                context().fonts().renderer("main").draw_string(30.0, &self.text, (200.0, 100.0), 0xffffffff);
                context().fonts().renderer("main").draw_string(30.0, &self.text, (200.0, 300.0), 0xffffffff);
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
                    let (width, height) = context().fonts().renderer("main").draw_string(40.0, format!("{:?}", mouse.pos()), el.bounds(), 0xffffffff);
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