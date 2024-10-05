use std::ptr::addr_of_mut;
use std::time::Instant;

use gl::types::*;
use glfw::{Context, fail_on_errors, Glfw, GlfwReceiver, PWindow, SwapInterval, WindowEvent, WindowMode};

use crate::components::bounds::Bounds;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::framework::framework::Framework;
use crate::components::render::font::FontManager;
use crate::components::render::renderer::Renderer;
use crate::components::window::Window;
use crate::components::wrapper::framebuffer::Framebuffer;
use crate::gl_binds::{gl11, gl20, gl30};
use crate::gl_binds::gl11::*;

static mut CONTEXT: Option<UIContext> = None;

pub unsafe fn context() -> &'static mut UIContext {
    match &mut CONTEXT {
        None => panic!("context was requested by was never created"),
        Some(context) => context,
    }
}

pub struct UIContext {
    glfw: Glfw,
    p_window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    framebuffer: Framebuffer,
    last_frame: Instant,

    window: Window,
    renderer: Renderer,
    font_manager: FontManager,
    framework: Framework,
}

impl UIContext {
    pub unsafe fn create_instance(width: i32, height: i32, title: impl ToString, mode: WindowMode) {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut p_window, events) = glfw.create_window(width as u32, height as u32, title.to_string().as_str(), mode).expect("Failed to make window");

        p_window.make_current();
        p_window.set_all_polling(true);

        gl30::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        gl11::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        gl20::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        gl::load_with(|f_name| glfw.get_proc_address_raw(f_name));

        CONTEXT = Some(UIContext {
            glfw,
            p_window,
            events,
            framebuffer: Framebuffer::new(RGBA, width, height).expect("Failed to create main framebuffer"),
            last_frame: Instant::now(),
            window: Window::new(width, height),
            renderer: Renderer::new(),
            font_manager: FontManager::new(""),
            framework: Framework::new(),
        });
        context().fonts().set_font_bytes("main", include_bytes!("../assets/fonts/ProductSans.ttf").to_vec());
    }

    pub unsafe fn do_loop(&mut self) {
        loop {
            self.handle();

            self.glfw.set_swap_interval(SwapInterval::Sync(1));
        }
    }

    pub unsafe fn handle(&mut self) {
        self.handle_events();

        self.pre_render();
        self.framework.event(Event::Render(RenderPass::Main));
        self.last_frame = Instant::now();
        check_error("render");
        self.post_render();
    }

    unsafe fn pre_render(&mut self) {
        Viewport(0, 0, context().window().width as GLsizei, context().window().height as GLsizei);

        check_error("pre");
        Clear(DEPTH_BUFFER_BIT);
        MatrixMode(PROJECTION);
        LoadIdentity();
        Ortho(0 as GLdouble, context().window().width as GLdouble, context().window().height as GLdouble, 0 as GLdouble, 1000 as GLdouble, 3000 as GLdouble);
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
        self.renderer.draw_texture_rect_uv(&Bounds::xywh(0.0, 0.0, context().window().width as f32, context().window().height as f32), &Bounds::ltrb(0.0, 1.0, 1.0, 0.0), 0xffffffff);
        self.framebuffer.unbind();

        self.renderer.end_frame();
        self.p_window.swap_buffers();
    }

    pub unsafe fn handle_events(&mut self) {
        self.glfw.poll_events();
        loop {
            match self.events.receive() {
                Some((_, event)) => {
                    self.window.handle(&event);
                }
                None => break
            }
        }
    }
    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
    pub fn fonts(&mut self) -> &mut FontManager {
        &mut self.font_manager
    }
    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }
    pub fn framework(&mut self) -> &mut Framework { &mut self.framework }
}

pub unsafe fn check_error(th: &str) {
    let mut err = GetError();
    while err != 0 {
        println!("{} OpenGL: {:?}", th, err);
        err = GetError();
    }
}