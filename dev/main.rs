#![allow(non_snake_case)]
//#![windows_subsystem = "windows"] // TO STOP EXTERNAL CONSOLE FROM OPENING WHEN RUNNING

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

extern crate alloc;

use ferrum::components::framework::layout::{LayoutContext, LayoutDirection, LayoutEvent};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Instant, UNIX_EPOCH};
use gl::Enable;

use glfw::{Action, Key, SwapInterval, WindowHint};
use num_traits::Pow;
use parking_lot::Mutex;
use winapi::um::wincon::FreeConsole;

use ferrum::components::context::{context, ContextBuilder};
use ferrum::components::editor::textbox::Textbox;
use ferrum::components::framework::animation::{Animation, AnimationRef, Easing};
use ferrum::components::framework::element::ElementBuilder;
use ferrum::components::framework::element::comp_element::CompElement;
use ferrum::components::framework::ui_traits::{TickResult, UIHandler, UIHandlerRef, UIIdentifier};
use ferrum::components::framework::event::{Event, RenderPass};
use ferrum::components::framework::layer::Layer;
use ferrum::components::framework::screen::ScreenTrait;
use ferrum::components::render::color::{solid, Color, To4Colors, ToColor};
use ferrum::components::render::font::format::{Alignment, DefaultFormatter, FormatItem, Text};
use ferrum::components::render::font::format::Alignment::{Left, Center, Right, Custom};
use ferrum::components::render::font::format::FormatItem::{AlignH, AlignV, LineSpacing, Size};
use ferrum::components::render::mask::FramebufferMask;
use ferrum::components::render::renderer::{shader_file, Renderable};
use ferrum::components::render::renderer::shapes::Rect;
use ferrum::components::spatial::vec2::Vec2;
use ferrum::components::spatial::vec4::Vec4;
use ferrum::components::wrapper::shader::Shader;
use ferrum::components::wrapper::texture::Texture;
use ferrum::gl_binds::gl11::{Finish, ALPHA, BLEND};
use ferrum::{container, element, fb_mask, register_anims, text};
use ferrum::components::framework::element::container::Container;
use ferrum::components::framework::layout::Sizing;

fn main() {
    // editor();
    let args : Vec<String> = std::env::args().collect();
    if !(args.len() > 1 && args[1] == "console") {
        unsafe {
            FreeConsole(); 
        }
    }

    unsafe {
        ContextBuilder::new()
            .title("Test")
            .dims(1920/2, 1080/2)
            .hint(WindowHint::Resizable(false))
            .swap_interval(SwapInterval::Adaptive)
            .build();

        context().fonts().set_font_source("main", include_bytes!("../src/assets/fonts/JetBrainsMono-Medium.ttf").to_vec());
        context().framework().set_screen(TestScreen::new());
        context().do_loop()
    }
}

pub struct TestScreen {
    pub text: String,
    previous_pos: Vec2<f32>,
    previous_tex: Arc<Mutex<String>>,
    t_size: AnimationRef,
    last_fps: u32,
    t_text: Arc<Mutex<Text>>,
    mask: FramebufferMask,
    t_shader: Shader,
    items: Arc<Mutex<Vec<Test>>>,
    test_shape: Rect,
    txt: Option<Rc<RefCell<Textbox>>>,
}

impl TestScreen {
    pub unsafe fn new() -> Self {
        // let mut t = include_str!("../test_4.js").to_string();
        let mut t = String::from_utf8(fs::read("dev/test.js").unwrap()).unwrap();
        // 5,320,704
        // println!("LEN : {}", t.len());
        // before
        // let bytes = fs::read("C:\\Windows\\Fonts\\consola.ttf").unwrap();
        // before
        // context().fonts().set_font_bytes("main", bytes);
        println!("{}", t.len());
        // context().fonts().load_font("main", true);

        let v = vec![Test::new(1), Test::new(2), Test::new(3), Test::new(4),Test::new(1000)];
        let test_vec = Arc::new(Mutex::new(v));
        TestScreen {
            text: t,
            previous_pos: Vec2::new(0.0, 0.0),
            previous_tex: Arc::new(Mutex::new("".to_string())),
            t_size: Animation::zero_ref(),
            last_fps: 0,
            t_text: Arc::new(Mutex::new(Text::new())),
            mask: FramebufferMask::new(),
            t_shader: Shader::new(shader_file("shaders/vertex.glsl"), shader_file("shaders/test.frag")),
            items: test_vec,
            test_shape: Rect::new(Vec4::xywh(100., 200., 200., 200.), solid(0xff00ff00)),
            txt: None,
        }
    }
}

impl ScreenTrait for TestScreen {
    unsafe fn handle(&mut self, event: &Event) { match event {
        Event::PreRender => {
            self.t_size.borrow_mut().animate(4f32, Easing::Sin);
        }
        Event::Keyboard(key, action, _) => {
            if action == &Action::Release {
                return;
            }
            let len = self.items.lock().len();
            let mut lock = self.items.lock();
            match key {
                Key::Backspace => {
                    lock.pop();
                }
                _ => {
                    let new = match lock.last() {
                        None => 0,
                        Some(v) => v.v + 1,
                    };
                    // lock.push(Test::new(new));
                }
            }
        }
        Event::Scroll(_, y) => {
            let current = self.t_size.borrow().target();
            self.t_size.borrow_mut().set_target(*y + current);
        }
        Event::Render(pass) => {
            if pass != &RenderPass::Main {
                return;
            }

            match &context().fonts().font("main").unwrap().atlas_tex {
                None => {}
                Some(t) => {
                    // Enable(BLEND);
                    // self.t_shader.bind();
                    // t.bind();
                    // let dim = context().window().height().min(context().window().width());
                    // context().renderer().draw_texture_rect(Vec4::xywh(10., 10., dim as f32, dim as f32), 0xffffffff);
                    // Texture::unbind();
                    // Shader::unbind();
                }
            }

            let mut fr = context().fonts().font("main").unwrap();

            let text: Text = text!(
                AlignH(Center),
                (20., "aligned middle? with some\nextra lines\nto spare", 0xffffffff),
                (36., "along with some\nLARGER ", 0xff00ffff),
                (18., "text...\n", 0xffffffff),
                Size(16.),
                AlignH(Left), "left\n",
                AlignH(Center), "center\n",
                AlignH(Right), "right\n",
                AlignH(Custom(0.75)), "0.75\n",
                AlignH(Custom(2.)), "this is a 2.",
                AlignH(Custom(-1.)), "this is a -1.\n",
                AlignH(Custom(2.)), "this is another 2.",
                AlignH(Custom(-1.)), "this is another -1.\n\n",
                AlignH(Right), "and now inline",
                AlignH(Center), "and it should work"
            );
            let fr_d = context().fonts().font("main").unwrap().draw_string(text, (500., 100.));

            fr.draw_string(text!(AlignH(Right), (20.0, format!("{:?}", context().fps()), 0xffffffff)), context().window().bounds().wh().offset_new((-4., -20.)));

            self.last_fps = context().fps();
            self.test_shape.render();

            // match &self.txt {
            //     None => {}
            //     Some(v) => {
            //         let text  = v.borrow().get_text();
            //         println!("{}", text);
            //     }
            // }
        }
        Event::PostRender => {
            self.previous_pos = *context().window().mouse().pos();
        }
        Event::MouseClick(_, _) => {}
        Event::Keyboard(_, _, _) => {}
        _ => {}
    }}

    unsafe fn init(&mut self) -> Vec<Layer> {
        register_anims! [
            self.t_size
        ];

        let mut layer_0 = Layer::new((32, 32));
        let mut layer_1 = Layer::new((32,32));
        let tex_cl1 = self.previous_tex.clone();
        let tex_cl2 = self.previous_tex.clone();
        let t_test_c = self.t_text.clone();

        let container = container! {
            layout: {
                size_behavior: (Sizing::Fixed(1000.), Sizing::Fixed(1000.)),
                // direction: LayoutDirection::Vertical,
                debug_color: Color::from_f32(1., 0., 0., 1.),
                // padding: Vec4::ltrb(5., 5., 5., 5.),
                spacing: (10., 200.).into(),
                pref_size: (1000., 1000.).into(),
                margin: Vec4::ltrb(10., 10., 10., 10.),
            },
            container! {
                layout: {
                    min_size: (200., 200.).into(),
                    margin: Vec4::ltrb(10., 10., 10., 10.),
                    debug_color: Color::from_f32(0., 1., 0., 1.),
                    spacing: (10., 0.).into(),
                    direction: LayoutDirection::Horizontal,
                },
                {
                    let mut rect = Rect::new(Vec4::xywh(10, 10, 10, 10), solid(0xffffffff));
                    element!(
                        layout: {
                            size_behavior: (Sizing::Grow, Sizing::Grow),
                        },
                        move |el, e| unsafe {
                            if e.is_prerender() {
                                rect.pre_render();
                            }
                            if e.is_render(RenderPass::Main) {
                                rect.set_bounds(el.bounds());
                                rect.render()
                            }
                        }
                    ).tick(|el, e| TickResult::RedrawLayout)
                    .build()
                },
                {
                    let mut rect = Rect::new(Vec4::xywh(10, 10, 10, 10), solid(0xffffffff));
                    element!(
                        layout: {
                            size_behavior: (Sizing::Grow, Sizing::Grow),
                        },
                        move |el, e| unsafe {
                            if e.is_prerender() {
                                rect.pre_render();
                            }
                            if e.is_render(RenderPass::Main) {
                                rect.set_bounds(el.bounds());
                                rect.render()
                            }
                        }
                    ).tick(|el, e| TickResult::RedrawLayout)
                    .build()
                },
                {
                    let mut rect = Rect::new(Vec4::xywh(10, 10, 10, 10), solid(0xffffffff));
                    element!(
                        layout: {
                            size_behavior: (Sizing::Grow, Sizing::Grow),
                        },
                        move |el, e| unsafe {
                            if e.is_prerender() {
                                rect.pre_render();
                            }
                            if e.is_render(RenderPass::Main) {
                                rect.set_bounds(el.bounds());
                                rect.render()
                            }
                        }
                    ).tick(|el, e| TickResult::RedrawLayout)
                    .build()
                },
            },
            container! {
                layout: {
                    min_size: (200., 200.).into(),
                    margin: Vec4::ltrb(10., 10., 10., 10.),
                    debug_color: Color::from_f32(0., 0., 1., 1.),
                    spacing: (0., 0.).into(),
                    direction: LayoutDirection::Horizontal,
                },
                {
                    let mut rect = Rect::new(Vec4::xywh(10, 10, 10, 10), solid(0xffffffff));
                    element!(
                        layout: {
                            size_behavior: (Sizing::Grow, Sizing::Grow),
                        },
                        move |el, e| unsafe {
                            if e.is_prerender() {
                                rect.pre_render();
                            }
                            if e.is_render(RenderPass::Main) {
                                rect.set_bounds(el.bounds());
                                rect.render()
                            }
                        }
                    ).tick(|el, e| TickResult::RedrawLayout)
                    .build()
                },
                {
                    let mut rect = Rect::new(Vec4::xywh(10, 10, 10, 10), solid(0xffffffff));
                    element!(
                        layout: {
                            size_behavior: (Sizing::Grow, Sizing::Grow),
                        },
                        move |el, e| unsafe {
                            if e.is_prerender() {
                                rect.pre_render();
                            }
                            if e.is_render(RenderPass::Main) {
                                rect.set_bounds(el.bounds());
                                rect.render()
                            }
                        }
                    ).tick(|el, e| TickResult::RedrawLayout)
                    .build()
                },
                {
                    let mut rect = Rect::new(Vec4::xywh(10, 10, 10, 10), solid(0xffffffff));
                    element!(
                        layout: {
                            size_behavior: (Sizing::Grow, Sizing::Grow),
                        },
                        move |el, e| unsafe {
                            if e.is_prerender() {
                                rect.pre_render();
                            }
                            if e.is_render(RenderPass::Main) {
                                rect.set_bounds(el.bounds());
                                rect.render()
                            }
                        }
                    ).tick(|el, e| TickResult::Redraw)
                    .build()
                },
            },
        };

        // let mut container = Container::new();
        //
        // let element = ;
        //
        // container.add(element);
        layer_1.add(container);
        // layer_0.add(el_test);
        // layer_0.add(el_1.build());
        let (ui, txt) = UIHandlerRef::new(Textbox::new("main", &"Ï".to_string()));
        layer_1.add(ui); // "".to_string() self.text.clone()
        self.txt = Some(txt);

        // layer
        self.text = "".to_string();

        // layer_0.elements().first().unwrap().

        vec![layer_0, layer_1]
        // vec![]
    }

    unsafe fn tick(&mut self, rp: &RenderPass) -> TickResult {
        // true
        if rp == &RenderPass::Main {
            let res = self.last_fps != context().fps();
            TickResult::RedrawLayout
        } else {
            TickResult::Valid
        }
    }
}

pub struct TestScreen2 {
    counter: Arc<Mutex<i32>>,
    input: Arc<Mutex<String>>,
}

impl TestScreen2 {
    pub unsafe fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            input: Arc::new(Mutex::new("test".to_string())),
        }
    }
}

impl ScreenTrait for TestScreen2 {
    unsafe fn handle(&mut self, event: &Event) {

    }

    unsafe fn init(&mut self) -> Vec<Layer> { unsafe {
        let mut layer = Layer::new((12, 12));

        let counter = self.counter.clone();
        let counter_text = self.counter.clone();

        // Counter text element
        layer.add(
            ElementBuilder::new()
                .bounds(Vec4::xywh(20.0, 20.0, 200.0, 40.0))
                .handler(move |el, event| {
                    if let Event::Render(pass) = event {
                        if pass == &RenderPass::Main {
                            let val = *counter_text.lock();
                            let mut fr = context().fonts().font("main").unwrap();
                            fr.draw_string(
                                (20.0, format!("Counter: {}", val), 0xffffffff),
                                el.bounds().pos(),
                            );
                        }
                    }
                })
                .build()
        );

        // Increment button
        let counter = self.counter.clone();
        layer.add(
            ElementBuilder::new()
                .bounds(Vec4::xywh(20.0, 70.0, 100.0, 40.0))
                .handler(move |el, event| {
                    match event {
                        Event::Render(pass) if pass == &RenderPass::Main => {
                            // context().renderer().draw_rounded_rect(el.bounds(), 5.0, 0xff2020ff);
                            // context().renderer().draw_rect(Vec4::xywh(200, 200, 1, 1), 0xffffffff);
                            let mut fr = context().fonts().font("main").unwrap();
                            fr.draw_string((20.0, "Increment", 0xffffffff), el.bounds().pos());
                        }
                        Event::MouseClick(_, action) => {
                            if el.hovering() && *action == Action::Press {
                                *counter.lock() += 1;
                            }
                        }
                        _ => {}
                    }
                })
                .draggable(false)
                .build()
        );

        let (ui_ref, txt_ref) = UIHandlerRef::new(Textbox::new("main", &self.input.lock().clone()));
        layer.add(ui_ref);

        vec![layer]
    }}

    unsafe fn tick(&mut self, render_pass: &RenderPass) -> TickResult {
        TickResult::RedrawLayout
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Test {
    v: i32,
}

impl Test {
    pub fn new(v: i32) -> Self {
        Test {
            v
        }
    }
}

impl UIIdentifier for Test {
    fn ui_id(&self) -> u64 {
        self.v as u64
    }
} 