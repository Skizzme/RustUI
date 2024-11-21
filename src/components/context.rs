use std::thread;
use std::time::{Duration, Instant};

use gl::types::*;
use glfw::{Context, fail_on_errors, Glfw, GlfwReceiver, PWindow, SwapInterval, WindowEvent, WindowHint, WindowMode};

use crate::components::framework::event::{Event, RenderPass};
use crate::components::framework::framework::Framework;
use crate::components::render::font::manager::FontManager;
use crate::components::render::renderer::Renderer;
use crate::components::render::stack::State;
use crate::components::window::Window;
use crate::components::wrapper::framebuffer::{Framebuffer, FramebufferManager};
use crate::components::wrapper::shader::Shader;
use crate::components::wrapper::texture::Texture;
use crate::gl_binds::{gl11, gl20, gl30, gl41};
use crate::gl_binds::gl11::*;
use crate::gl_binds::gl20::{ActiveTexture, TEXTURE0};

static mut CONTEXT: Option<UIContext> = None;

#[allow(static_mut_refs)]
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
    framebuffer: u32,
    frames: (u32, u32, Instant),
    last_render: Instant,
    content_scale: (f32, f32),

    window: Window,
    renderer: Renderer,
    font_manager: FontManager,
    framework: Framework,
    fb_manager: FramebufferManager,

    close_requested: bool,
}

impl UIContext {
    pub unsafe fn create_instance(builder: ContextBuilder) {
        let mut glfw = builder.glfw;
        let (mut p_window, events) = glfw.create_window(builder.width as u32, builder.height as u32, builder.title.as_str(), builder.mode).expect("Failed to make window");

        p_window.make_current();
        p_window.set_all_polling(true);

        gl41::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        gl30::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        gl11::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        gl20::load_with(|f_name| glfw.get_proc_address_raw(f_name));
        gl::load_with(|f_name| glfw.get_proc_address_raw(f_name));

        let fb_manager = FramebufferManager::new();
        CONTEXT = Some(UIContext {
            glfw,
            p_window,
            events,
            framebuffer: 0,
            frames: (0, 0, Instant::now()),
            last_render: Instant::now(),
            content_scale: (1.0, 1.0),
            window: Window::new(builder.width, builder.height),
            renderer: Renderer::new(),
            font_manager: FontManager::new(""),
            framework: Framework::new(),
            fb_manager,
            close_requested: false,
        });
        context().framebuffer = context().fb_manager.create_fb(RGBA).unwrap();
    }

    pub unsafe fn do_loop(&mut self) {
        while !self.close_requested {
            if !self.frame() {
                thread::sleep(Duration::from_secs_f32(1.0/200.0));
            }

            if self.last_render.elapsed().as_secs_f32() > 1.0 {
                thread::sleep(Duration::from_millis(50));
            }
            // Finish();
            self.glfw.set_swap_interval(SwapInterval::Adaptive);
            // self.glfw.set_swap_interval(SwapInterval::None);
        }
    }

    pub unsafe fn frame(&mut self) -> bool {

        self.handle_events();
        self.window.mouse.frame();

        self.framework.event(Event::PreRender);
        let should_render = self.should_render();
        if should_render {
            self.render();
            self.last_render = Instant::now();
        }
        self.framework.event(Event::PostRender);
        should_render
    }

    pub unsafe fn should_render(&mut self) -> bool {
        self.framework.should_render_all()
    }

    pub unsafe fn render(&mut self) {
        self.pre_render();
        Finish();

        PushMatrix();
        self.renderer.stack().push(State::Scale(self.content_scale.0, self.content_scale.1));
        context().window().mouse.pos /= (self.content_scale.0, self.content_scale.1);

        let mut passes = RenderPass::all();
        // passes.reverse(
        for pass in passes {
            // if pass != RenderPass::Main {
            //     continue
            // }
            let (parent_fb, parent_tex) = self.framework.element_pass_fb(&pass).bind();
            if self.framework.should_render_pass(&pass) {
                Framebuffer::clear_current();
                // self.framework.pass_fb(&pass).copy_from_parent();
                self.framework.event(Event::Render(pass.clone()));

                match pass {
                    RenderPass::Main => {}
                    RenderPass::Bloom => {
                        if self.renderer.blur_fb ==  0{
                            self.renderer.blur_fb = self.fb_manager.create_fb(RGBA).unwrap();
                        }

                        let (mut parent_fb_2, mut parent_tex_2) = (0, 0);
                        {
                            let blur_fb = self.fb_manager.fb(self.renderer.blur_fb);
                            // blur_fb.tex_filter();
                            (parent_fb_2, parent_tex_2) = blur_fb.bind();
                            Framebuffer::clear_current();
                            // blur_fb.copy_from_parent();
                        }
                        // let shader = &mut ;
                        self.renderer.bloom_shaders.0.bind();

                        let mut last_tex = parent_tex_2;
                        for i in 0..3 {
                            {
                                let shader = &self.renderer.bloom_shaders.0;
                                shader.u_put_float("offset", vec![4.0 / (i as f32 + 1.0), 4.0 / (i as f32 + 1.0)]);
                                shader.u_put_float("half_pixel", vec![1.0 / self.window.width as f32, 1.0 / self.window.height as f32]);
                                shader.u_put_float("resolution", vec![self.window.width as f32, self.window.height as f32]);
                                shader.u_put_int("texture", vec![0]);
                                shader.u_put_float("noise", vec![0.2]);
                                shader.u_put_int("check", vec![1]);
                                shader.u_put_int("check_texture", vec![1]);
                            }
                            // ActiveTexture(TEXTURE1);
                            // BindTexture(TEXTURE_2D, last_tex as GLuint);
                            ActiveTexture(TEXTURE0);
                            BindTexture(TEXTURE_2D, last_tex as GLuint);
                            self.renderer.draw_screen_rect_flipped();
                            last_tex = self.fb_manager.fb(self.renderer.blur_fb).texture_id() as i32;
                        }
                        self.renderer.bloom_shaders.1.bind();

                        let mut last_tex = parent_tex_2;
                        for i in 0..3 {
                            {
                                let shader = &self.renderer.bloom_shaders.1;
                                shader.u_put_float("offset", vec![4.0 / (i as f32 + 1.0), 4.0 / (i as f32 + 1.0)]);
                                shader.u_put_float("half_pixel", vec![1.0 / self.window.width as f32, 1.0 / self.window.height as f32]);
                                shader.u_put_float("resolution", vec![self.window.width as f32, self.window.height as f32]);
                                shader.u_put_int("texture", vec![0]);
                                shader.u_put_float("noise", vec![0.2]);
                                shader.u_put_int("check", vec![1]);
                                shader.u_put_int("check_texture", vec![1]);
                            }
                            // ActiveTexture(TEXTURE1);
                            // BindTexture(TEXTURE_2D, last_tex as GLuint);
                            ActiveTexture(TEXTURE0);
                            BindTexture(TEXTURE_2D, last_tex as GLuint);
                            self.renderer.draw_screen_rect_flipped();
                            last_tex = self.fb_manager.fb(self.renderer.blur_fb).texture_id() as i32;
                        }

                        self.fb_manager.fb(self.renderer.blur_fb).unbind();
                        Framebuffer::clear_current(); // clear pass buffer
                        // self.fb_manager.fb(self.renderer.blur_fb).bind_texture();
                        Shader::unbind();

                        // self.renderer.draw_screen_rect_flipped();
                        Texture::unbind();
                    }
                    RenderPass::Post => {}
                    RenderPass::Custom(_) => {}
                }
            }
            match pass {
                RenderPass::Main => {}
                RenderPass::Bloom => {
                    Framebuffer::clear_current();
                    self.fb_manager.fb(self.renderer.blur_fb).bind_texture();
                    self.renderer.draw_screen_rect_flipped();
                    // self.fb_manager.fb(self.renderer.blur_fb).copy_bind(parent_fb as u32, parent_tex as u32);
                }
                RenderPass::Post => {}
                RenderPass::Custom(_) => {}
            }

            self.framework.element_pass_fb(&pass).copy_bind(parent_fb as u32, parent_tex as u32);
        }
        // for pass in RenderPass::all().iter().rev() {
        //     Enable(BLEND);
        //     self.framework.pass_fb(pass).bind_texture();
        //     self.renderer.draw_screen_rect_flipped();
        // }
        // Texture::unbind();
        // Finish();
        // println!("render took {:?}", st.elapsed());

        context().window().mouse.pos *= (self.content_scale.0, self.content_scale.1);
        self.renderer.stack().pop();
        PopMatrix();
        Finish();
        check_error("render");
        self.post_render();

        self.font_manager.cleanup();
        self.frames.0 += 1;
        if self.frames.2.elapsed().as_secs_f32() >= 1.0 {
            self.frames.1 = self.frames.0;
            self.frames.0 = 0;
            self.frames.2 = Instant::now();
        }
        // println!("frame");
    }

    unsafe fn pre_render(&mut self) {
        Viewport(0, 0, context().window().width as GLsizei, context().window().height as GLsizei);

        Clear(DEPTH_BUFFER_BIT);
        MatrixMode(PROJECTION);
        LoadIdentity();
        Ortho(0 as GLdouble, context().window().width as GLdouble, context().window().height as GLdouble, 0 as GLdouble, 1000 as GLdouble, 3000 as GLdouble);
        Translated(0 as GLdouble, 0 as GLdouble, -2000 as GLdouble);

        Clear(COLOR_BUFFER_BIT);
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        self.framebuffer().bind();
        Framebuffer::clear_current();
        check_error("pre");
    }

    unsafe fn post_render(&mut self) {
        check_error("post");
        self.framebuffer().unbind();
        self.framebuffer().bind_texture();
        self.renderer.draw_screen_rect_flipped();
        self.framebuffer().unbind();

        self.renderer.end_frame();
        self.p_window.swap_buffers(); }

    pub unsafe fn handle_events(&mut self) {
        self.glfw.poll_events();
        loop {
            match self.events.receive() {
                Some((_, event)) => {
                    self.window.handle(&event);
                    match &event {
                        WindowEvent::Key(key, code, action, mods) => {
                            self.framework.event(Event::Keyboard(*key, *action, *mods));
                        }
                        WindowEvent::Size(width, height) => {
                            self.fb_manager().resize(*width, *height);
                            self.framework.on_resize(*width as f32, *height as f32);
                        }
                        WindowEvent::Scroll(x, y) => {
                            self.framework.event(Event::Scroll((*x) as f32, (*y) as f32))
                        }
                        WindowEvent::CursorPos(x, y) => {
                            self.framework.event(Event::PreRender);
                            self.framework.event(Event::MousePos(*x as f32, *y as f32));
                        }
                        WindowEvent::Close => self.close_requested = true,
                        WindowEvent::MouseButton(button, action, _) => {
                            self.framework.event(Event::MouseClick(*button, *action))
                        }
                        WindowEvent::ContentScale(x, y) => {
                            println!("xy {} {}", x, y);
                            // self.p_window.set_size((self.window.width as f32 * (x / self.content_scale.0)) as i32, (self.window.height as f32 * (y / self.content_scale.1)) as i32);
                            // self.content_scale = (*x, *y)
                        }
                        WindowEvent::FileDrop(fl) => {
                            println!("{:?}", fl);
                        }
                        _ => {}
                    }
                }
                None => break
            }
        }
    }

    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
    pub fn fonts(&mut self) -> &mut FontManager { &mut self.font_manager }
    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }
    pub fn framework(&mut self) -> &mut Framework { &mut self.framework }
    pub fn fps(&self) -> u32 { self.frames.1 }
    pub fn p_window(&mut self) -> &mut PWindow { &mut self.p_window }
    pub fn fb_manager(&mut self) -> &mut FramebufferManager { &mut self.fb_manager }
    pub unsafe fn framebuffer(&mut self) -> &mut Framebuffer { self.fb_manager.fb(self.framebuffer) }
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
            glfw: glfw::init(|e, s| eprintln!("{:?} {:?}", e, s)).unwrap(),
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