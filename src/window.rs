use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

pub struct Window {
    pub screen_width: u32,
    pub screen_height: u32,
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
    pub unsafe fn create(width: u32, height: u32) -> Window {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(WindowHint::ContextVersion(4, 6));
        glfw.window_hint(WindowHint::Resizable(false));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Any));
        glfw.window_hint(WindowHint::Samples(Some(8u32)));

        let (mut p_window, events) = glfw.create_window(width, height, TITLE, glfw::WindowMode::Windowed).expect("Failed to make window");

        p_window.make_current();
        p_window.set_all_polling(true);

        gl11::load_with(|f_name| {
            glfw.get_proc_address_raw(f_name)
        });
        gl::load_with(|f_name| {
            glfw.get_proc_address_raw(f_name)
        });
        gl20::load_with(|f_name| {
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
                _e => {}
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