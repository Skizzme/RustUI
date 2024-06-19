use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

use gl::*;
use gl::types::*;
use glfw::{Context, fail_on_errors, Glfw, GlfwReceiver, PWindow, SwapInterval, WindowEvent, WindowHint, WindowMode};
use crate::components::events::KeyboardEvent;

use crate::components::render::bounds::Bounds;
use crate::components::render::font::FontManager;
use crate::components::render::renderer::Renderer;
use crate::components::screen::{Element, ScreenTrait};
use crate::components::wrapper::framebuffer::Framebuffer;
use crate::gl_binds::gl30;
use crate::gl_binds::gl30::{LoadIdentity, MatrixMode, Ortho, PROJECTION, Translated};

/// A wrapper for the GLFW window
///
/// Contains all necessary variables, and should be the global window object of the program
pub struct Window {
    pub width: i32,
    pub height: i32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub frame_delta: f64,
    pub renderer: Rc<Renderer>,
    pub fonts: FontManager,

    pub p_window: PWindow,
    glfw: Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
    framebuffer: Framebuffer,
    unfocused_fps: u32
}

impl Window {
    /// Creates a new window with the specified options
    pub unsafe fn create(title: impl ToString, width: i32, height: i32, font_location: impl ToString, cache_location: impl ToString, glfw_hints: Vec<WindowHint>, mode: WindowMode, unfocused_fps: u32) -> Window {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        for glfw_hint in glfw_hints {
            glfw.window_hint(glfw_hint);
        }

        let (mut p_window, events) = glfw.create_window(width as u32, height as u32, title.to_string().as_str(), mode).expect("Failed to make window");

        p_window.make_current();
        p_window.set_all_polling(true);

        gl30::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        load_with(|f_name| glfw.get_proc_address_raw(f_name));

        let renderer = Rc::new(Renderer::new());

        Window {
            width,
            height,
            mouse_x: 0.0,
            mouse_y: 0.0,
            frame_delta: 0.0,
            renderer: renderer.clone(),
            fonts: FontManager::new(width, height, renderer.clone(), font_location, cache_location),
            p_window,
            glfw,
            events,
            unfocused_fps,
            framebuffer: Framebuffer::new(RGBA, width, height).expect("Failed to create main framebuffer"),
        }
    }

    /// The method that should be called every frame.
    ///
    /// Polls events, tracks frame_delta, and calls `draw` on `current_screen`
    #[allow(unused_mut)]
    pub unsafe fn frame(&mut self, mut current_screen: Box<&mut dyn ScreenTrait>, last_frame: Instant) {
        self.glfw.poll_events();
        let mut keyboard_events = Vec::new();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::CursorPos(x, y) => {
                    self.mouse_x = x as f32;
                    self.mouse_y = y as f32;
                }
                WindowEvent::Key(key, code, action, mods) => {
                    current_screen.key_press(key, code, action, mods);
                    keyboard_events.push(KeyboardEvent::new(key, code, action, mods));
                    // for i in 0..current_screen.screen().keyboard_inputs.len() {
                    //     let input = current_screen.screen().keyboard_inputs[i];
                    //     input.key_action(self.(), KeyboardEvent::new(key, code, action, mods))
                    // }
                    // for mut input in current_screen.screen().keyboard_inputs {
                        // let a = self;
                    // }
                },
                WindowEvent::Size(width, height) => {
                    self.width = width;
                    self.height = height;
                    self.fonts.screen_width = width;
                    self.fonts.screen_height = height;
                }
                e => {
                    current_screen.event(e, self);
                }
            }
        }

        self.pre_render();

        if !self.p_window.is_focused() {
            self.glfw.set_swap_interval(SwapInterval::Sync(0));
            let target_delta = 1.0/self.unfocused_fps as f32;
            thread::sleep(Duration::from_secs_f32(target_delta));
        } else {
            self.glfw.set_swap_interval(SwapInterval::Sync(1));
        }

        current_screen.draw(self);
        // Will also draw elements
        for e in current_screen.elements().iter() {
            match e {
                Element::Drawable(drawable) => {
                    drawable.lock().unwrap().draw(self);
                }
                Element::KeyboardReceiver(r) => {
                    for key in keyboard_events.iter() {
                        r.lock().unwrap().key_action(self, key)
                    }
                }
                Element::MouseInputs(_) => {}
            }
        }

        self.post_render();
        self.frame_delta = last_frame.elapsed().as_secs_f64();
    }

    unsafe fn pre_render(&mut self) {
        Viewport(0, 0, self.width as GLsizei, self.height as GLsizei);

        check_error("pre");
        Clear(DEPTH_BUFFER_BIT);
        MatrixMode(PROJECTION);
        LoadIdentity();
        Ortho(0 as GLdouble, self.width as GLdouble, self.height as  GLdouble, 0 as GLdouble, 1000 as GLdouble, 3000 as GLdouble);
        Translated(0 as GLdouble, 0 as GLdouble, -2000 as GLdouble);

        Clear(COLOR_BUFFER_BIT);
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        self.framebuffer.bind();
        self.framebuffer.clear();
    }

    unsafe fn post_render(&mut self) {
        check_error("post");
        self.framebuffer.unbind();
        self.framebuffer.bind_texture();
        self.renderer.draw_texture_rect_uv(&Bounds::from_xywh(0.0, 0.0, self.width as f32, self.height as f32), &Bounds::from_ltrb(0.0, 1.0, 1.0, 0.0), 0xffffffff);
        self.framebuffer.unbind();

        self.p_window.swap_buffers();
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }
}

pub unsafe fn check_error(th: &str) {
    let mut err = GetError();
    while err != 0 {
        println!("{} OpenGL: {:?}", th, err);
        err = GetError();
    }
}