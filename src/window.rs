use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
// use gl::*;
// use gl::types::{GLdouble, GLsizei};
use glfw::{Context, fail_on_errors, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, SwapInterval, WindowEvent, WindowHint};
use image::open;
use crate::{BACKGROUND_FPS, gl30, TITLE};
use crate::font::FontManager;
use crate::gl30::*;
use crate::gl30::types::*;
use crate::renderer::Renderer;
use crate::screen::GuiScreen;
use crate::texture::Texture;

pub struct Window {
    pub screen_width: i32,
    pub screen_height: i32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub frame_delta: f64,
    pub renderer: Rc<Renderer>,
    pub fonts: FontManager,

    // pub current_screen: Box<dyn GuiScreen>,

    pub(crate) p_window: PWindow,
    glfw: Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub unsafe fn create(width: i32, height: i32) -> Window {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        // glfw.window_hint(WindowHint::ContextVersion(3, 2));
        // glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Compat));
        glfw.window_hint(WindowHint::Samples(Some(8u32)));
        // glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        // glfw.window_hint(WindowHint::DoubleBuffer(true)); // less tearing?
        // glfw.window_hint(WindowHint::Resizable(false));
        // glfw.window_hint(WindowHint::Floating(true));
        // glfw.window_hint(WindowHint::TransparentFramebuffer(true));

        let (mut p_window, events) = glfw.create_window(width as u32, height as u32, TITLE, glfw::WindowMode::Windowed).expect("Failed to make window");

        p_window.make_current();
        p_window.set_all_polling(true);

        // gl11::load_with(|f_name| {
        //     glfw.get_proc_address_raw(f_name)
        // });
        // gl::load_with(|f_name| {
        //     glfw.get_proc_address_raw(f_name)
        // });
        gl30::load_with(|f_name| {
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
                WindowEvent::Size(width, height) => {
                    self.screen_width = width;
                    self.screen_height = height;
                }
                e => {
                    current_screen.event(e, self);
                }
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


unsafe fn pre_render(window: &Window) {
    Viewport(0, 0, window.screen_width as GLsizei, window.screen_height as GLsizei);

    check_error("pre");
    Clear(DEPTH_BUFFER_BIT);
    MatrixMode(PROJECTION);
    LoadIdentity();
    Ortho(0 as GLdouble, window.screen_width as GLdouble, window.screen_height as  GLdouble, 0 as GLdouble, 1000 as GLdouble, 3000 as GLdouble);
    Translated(0 as GLdouble, 0 as GLdouble, -2000 as GLdouble);

    Clear(COLOR_BUFFER_BIT);
    BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    // Enable(MULTISAMPLE);
}

unsafe fn post_render(window: &mut PWindow) {
    check_error("post");
    window.swap_buffers();
}

pub unsafe fn check_error(th: &str) {
    let mut err = GetError();
    while err != 0 {
        println!("{} OpenGL: {:?}", th, err);
        err = GetError();
    }
}