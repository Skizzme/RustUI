use std::ptr::addr_of_mut;
use std::thread;
use std::time::{Duration, Instant};

use gl::types::*;
use glfw::{Context, fail_on_errors, Glfw, GlfwReceiver, PWindow, SwapInterval, WindowEvent, WindowHint, WindowMode};
use image::open;

use crate::components::bounds::Bounds;
use crate::components::framework::event::{Event, RenderPass};
use crate::components::framework::framework::Framework;
use crate::components::render::font::FontManager;
use crate::components::render::renderer::Renderer;
use crate::components::render::stack::State;
use crate::components::window::Window;
use crate::components::wrapper::framebuffer::Framebuffer;
use crate::components::wrapper::texture::Texture;
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
    frames: (u32, u32, Instant),
    content_scale: (f32, f32),
    last_frame: Instant,

    window: Window,
    renderer: Renderer,
    font_manager: FontManager,
    framework: Framework,

    close_requested: bool,
    pub tex: Texture,
}

impl UIContext {
    pub unsafe fn create_instance(builder: ContextBuilder) {
        let img = open("C:\\Users\\farre\\Pictures\\an event about to occur.png").unwrap().into_rgba8();
        let mut glfw = builder.glfw;
        let (mut p_window, events) = glfw.create_window(builder.width as u32, builder.height as u32, builder.title.as_str(), builder.mode).expect("Failed to make window");

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
            framebuffer: Framebuffer::new(RGBA, builder.width, builder.height).expect("Failed to create main framebuffer"),
            frames: (0, 0, Instant::now()),
            content_scale: (1.0, 1.0),
            last_frame: Instant::now(),
            window: Window::new(builder.width, builder.height),
            renderer: Renderer::new(),
            font_manager: FontManager::new(""),
            framework: Framework::new(),
            close_requested: false,
            tex: Texture::create(img.width() as i32, img.height() as i32, &img.into_raw(), RGBA),
        });
        context().fonts().set_font_bytes("main", include_bytes!("../assets/fonts/ProductSans.ttf").to_vec());
    }

    pub unsafe fn do_loop(&mut self) {
        while !self.close_requested {
            self.frame();

            // thread::sleep(Duration::from_millis(1000));
            // Finish();
            self.glfw.set_swap_interval(SwapInterval::Adaptive);
        }
    }

    pub unsafe fn frame(&mut self) {
        self.handle_events();
        self.window.mouse.frame();

        self.pre_render();
        self.renderer.stack().push(State::Scale(self.content_scale.0, self.content_scale.1));
        self.framework.event(Event::Render(RenderPass::Main));
        self.renderer.stack().pop();
        self.last_frame = Instant::now();
        check_error("render");
        self.post_render();

        self.font_manager.cleanup();
        self.frames.0 += 1;
        if self.frames.2.elapsed().as_secs_f32() >= 1.0 {
            self.frames.1 = self.frames.0;
            self.frames.0 = 0;
            self.frames.2 = Instant::now();
        }
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
                    match &event {
                        WindowEvent::Close => self.close_requested = true,
                        WindowEvent::MouseButton(button, action, mods) => {
                            self.framework.event(Event::MouseClick(*button, *action))
                        }
                        WindowEvent::ContentScale(x, y) => {
                            self.p_window.set_size((self.window.width as f32 * (x / self.content_scale.0)) as i32, (self.window.height as f32 * (y / self.content_scale.1)) as i32);
                            self.content_scale = (*x, *y)
                        }
                        WindowEvent::FileDrop(fl) => {
                            println!("{:?}", fl);
                        }
                        _ => {}
                    }
                    self.window.handle(&event);
                    // self.framework.event(Event::GlfwRaw(event));
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
    pub fn fps(&self) -> u32 {
        self.frames.1
    }
    pub fn p_window(&mut self) -> &mut PWindow {
        &mut self.p_window
    }
}

pub struct ContextBuilder<'a> {
    hints: Vec<WindowHint>,
    width: i32, height: i32,
    title: String,
    glfw: Glfw,
    mode: WindowMode<'a>,
}

impl<'a> ContextBuilder<'a> {
    pub fn new() -> ContextBuilder<'a> {
        ContextBuilder {
            hints: vec![],
            width: 400,
            height: 300,
            title: "".to_string(),
            glfw: glfw::init(fail_on_errors!()).unwrap(),
            mode: WindowMode::Windowed,
        }
    }

    pub fn title(&mut self, title: impl ToString) { self.title = title.to_string(); }
    pub fn dims(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    pub fn hints(&mut self, mut hints: Vec<WindowHint>) { self.hints.append(&mut hints); }
    pub fn hint(&mut self, hint: WindowHint) { self.hints.push(hint); }

    pub fn mode(&mut self, mode: WindowMode<'a>) { self.mode = mode; }

    pub unsafe fn build(self) {
        UIContext::create_instance(self);
    }
}

pub unsafe fn check_error(th: &str) {
    let mut err = GetError();
    while err != 0 {
        println!("{} OpenGL: {:?}", th, err);
        err = GetError();
    }
}