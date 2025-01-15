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

use std::cell::RefCell;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Add;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Instant, UNIX_EPOCH};

use glfw::{Action, Key, WindowHint};
use parking_lot::Mutex;
use winapi::um::wincon::FreeConsole;

use RustUI::components::context::{context, ContextBuilder};
use RustUI::components::framework::animation::{Animation, AnimationRef, AnimationType};
use RustUI::components::framework::element::ElementBuilder;
use RustUI::components::framework::element::comp_element::CompElement;
use RustUI::components::framework::element::ui_traits::{UIHandler, UIIdentifier};
use RustUI::components::framework::event::{Event, RenderPass};
use RustUI::components::framework::layer::Layer;
use RustUI::components::framework::screen::ScreenTrait;
use RustUI::components::render::color::ToColor;
use RustUI::components::render::font::format::{Alignment, DefaultFormatter, FormatItem, Text};
use RustUI::components::render::font::format::Alignment::{Left, Center, Right, Custom};
use RustUI::components::render::font::format::FormatItem::{AlignH, Color, LineSpacing, Size};
use RustUI::components::render::mask::FramebufferMask;
use RustUI::components::render::renderer::shader_file;
use RustUI::components::spatial::vec2::Vec2;
use RustUI::components::spatial::vec4::Vec4;
use RustUI::components::wrapper::shader::Shader;
use RustUI::text;

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
    previous_pos: Vec2,
    previous_tex: Arc<Mutex<String>>,
    t_size: AnimationRef,
    last_fps: u32,
    t_text: Arc<Mutex<Text>>,
    mask: FramebufferMask,
    t_shader: Shader,
    items: Arc<Mutex<Vec<Test>>>,
}

impl TestScreen {
    pub unsafe fn new() -> Self {
        let mut t = include_str!("../test.js").to_string();
        // t.push_str(&t.clone());
        // t.push_str(&t.clone());
        // t.push_str(&t.clone());
        // t.push_str(&t.clone());
        // t.push_str(&t.clone());
        // t.push_str(&t.clone());
        // t.push_str(&t.clone());
        // t.push_str(&t.clone());
        println!("LEN : {}", t.len());

        let bytes = fs::read("C:\\Windows\\Fonts\\consola.ttf").unwrap();

        context().fonts().set_font_bytes("main", bytes);
        // context().fonts().set_font_bytes("main", include_bytes!("assets/fonts/JetBrainsMono-Medium.ttf").to_vec());
        // context().fonts().load_font("main", true);

        let v = vec![Test::new(1), Test::new(2), Test::new(3), Test::new(4),Test::new(1000)];
        let test_vec = Arc::new(Mutex::new(v));
        TestScreen {
            text: t,
            previous_pos: Vec2::new(0.0, 0.0),
            previous_tex: Arc::new(Mutex::new("".to_string())),
            t_size: Rc::new(RefCell::new(Animation::zero())),
            last_fps: 0,
            t_text: Arc::new(Mutex::new(Text::new())),
            mask: FramebufferMask::new(),
            t_shader: Shader::new(shader_file("shaders/vertex.glsl"), shader_file("shaders/test.frag")),
            items: test_vec,
        }
    }
}

impl ScreenTrait for TestScreen {
    unsafe fn handle(&mut self, event: &Event) { match event {
        Event::PreRender => {
            self.t_size.borrow_mut().animate(4f32, AnimationType::Sin);
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
                    lock.push(Test::new(new));
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
            // context().renderer().draw_rect(context().window().bounds(), 0xff181818);

            let mut fr = context().fonts().font("main").unwrap();

            let text: Text = text!(
                AlignH(Center),
                (20., "aligned middle? with some\nextra lines\nto spare", 0xffffffff),
                (36., "along with some\nLARGER ", 0xffffffff),
                (18., "text...\n", 0xffffffff),
                AlignH(Left),
                Size(16.),
                "left\n",
                AlignH(Center),
                "center\n",
                AlignH(Right),
                "right\n",
                AlignH(Custom(0.75)),
                "0.75\n",
                AlignH(Custom(2.)),
                "this is a 2.",
                AlignH(Custom(-1.)),
                "this is a -1.\n",
                AlignH(Custom(2.)),
                "this is another 2.",
                AlignH(Custom(-1.)),
                "this is another -1.\n\n",
                AlignH(Right),
                "and now inline",
                AlignH(Center),
                "and it should work"
            );
            gl::Finish();
            let st = Instant::now();
            let (end_pos, bounds) = context().fonts().font("main").unwrap().draw_string(text, (500., 100.));
            gl::Finish();
            let et = st.elapsed();
            println!("drawed {:?}", et);
            context().renderer().draw_rect(Vec4::xywh(500., 100., 2., bounds.height()), 0xffffffff);

            context().renderer().draw_rect(Vec4::ltrb(10.0, 10.0, 200.0, 200.0), 0x90ff0000);
            fr.draw_string((30.0, format!("{:?}", context().fps()), 0xffffffff), (300, 100.0));
            let formated: Text = (15, "context().fonts().set_font_bytes(\"main\", include_bytes!(\"assets/fonts/JetBrainsMono-Medium.ttf\").to_vec());", 0xffffffff).into();
            let pos = 0; //
            fr.draw_string(formated, (pos, 300.0));
            self.last_fps = context().fps();
        }
        Event::PostRender => {
            self.previous_pos = *context().window().mouse().pos();
        }
        Event::MouseClick(_, _) => {}
        Event::Keyboard(_, _, _) => {}
        _ => {}
    }}

    unsafe fn init(&mut self) -> Vec<Layer> {
        context().framework().screen_animations().register(self.t_size.clone());
        let mut layer_0 = Layer::new();
        let tex_cl1 = self.previous_tex.clone();
        let tex_cl2 = self.previous_tex.clone();
        let t_test_c = self.t_text.clone();
        let el_1 = ElementBuilder::new()
            .bounds(Vec4::xywh(5.0, 100.0, 100.0, 100.0))
            .draggable(true)
            .handler(move |el, event| { match event {
                Event::MousePos(x, y) => {
                    *t_test_c.lock() = text!(
                        (36.0, "before", 0xffffff90),
                        (context().window().mouse().pos().x() / 400.0 * 40.0, format!("&ff2030ff{} {}", x, y), 0xffffffff),
                        (36.0, "after ", 0xff90ffff),
                        (20.0, format!("{}", UNIX_EPOCH.elapsed().unwrap().as_secs_f64()), 0)
                    ).with_formatter(&mut DefaultFormatter::new());
                }
                Event::Render(pass) => {
                    if pass != &RenderPass::Main {
                        return;
                    }
                    let mouse = context().window().mouse();
                    // context().renderer().draw_rect(*el.vec4(), 0xff00ff00);
                    let st = Instant::now();
                    let (end_pos, vec4) = context().fonts().font("main").unwrap().draw_string(t_test_c.lock().clone(), el.bounds());
                    // println!("{:?}", st.elapsed());
                    el.bounds().set_width(vec4.width());
                    el.bounds().set_height(vec4.height());
                    let hovering = el.hovering();
                    el.bounds().debug_draw(if hovering { 0xff10ff10 } else { 0xffffffff });

                    *tex_cl1.lock() = format!("{:?}", context().window().mouse().pos()).to_string();
                }
                _ => {}
            }})
            .should_render(move |_, rp| {
                // println!("el p check {:?}", rp);
                if rp == &RenderPass::Main {
                    let mouse = context().window().mouse();
                    let res = tex_cl2.lock().clone() != format!("{:?}", mouse.pos()).to_string();
                    res
                } else {
                    false
                }
            });

        let el_1_c = ElementBuilder::new()
            .bounds(Vec4::xywh(0.0, 0.0, 40.0, 40.0))
            .draggable(false)
            .handler(|el, event| { match event {
                Event::Render(pass) => {
                    // context().renderer().draw_rect(*el.vec4(), 0xff90ff20);z
                    // let hovering = el.hovering();
                    if pass == &RenderPass::Bloom {
                        let mut padded = el.bounds().clone();
                        padded.padded(10.0);
                        context().renderer().draw_rect(*el.bounds(), 0xffffffff);
                        context().renderer().draw_rect(padded, 0xff10ff10);
                        // el.vec4().draw_vec4(if hovering { 0xff10ff10 } else { 0xffffffff });
                    }
                },
                Event::MouseClick(_, action) => {
                    if el.hovering() && *action == Action::Press {
                        let v = !context().p_window().uses_raw_mouse_motion();
                        println!("change {}", v);
                        context().p_window().set_raw_mouse_motion(v);
                        // let v = !context().p_window().is_decorated();
                        // context().p_window().set_decorated(v);
                    }
                }
                    _ => {}
            }})
            .should_render(|_, _| {
                // println!("c el check");
                false
            });

        // let el_1 = el_1.child(el_1_c.build());

        let test_vec_1 = self.items.clone();
        let test_vec_2 = self.items.clone();

        let pos_state = Rc::new(RefCell::new(Vec2::zero()));
        let pos_state_1 = pos_state.clone();
        let el_test = CompElement::new(
            move |mut inner| {
                let mut lock = test_vec_2.lock();
                let mut i = 0;
                let mut state = Vec2::new(0.0, 0.0);
                pos_state.replace(Vec2::zero());
                for item in lock.iter_mut() {
                    inner(&mut state, item);
                    i += 1;
                    // state.set_x((i % 10 * 20) as f32);
                    // state.set_y(((i / 10) * 20) as f32);
                }
            },
            move |exists, state, item| {
                let num = item.v;
                let state_c = pos_state_1.clone();
                if !exists {
                    let mut el = ElementBuilder::new()
                        .handler(move |el, e| {
                            if e.is_render(RenderPass::Main) {
                                let mut fr = context().fonts().font("main").unwrap();
                                let (pos, bounds) = fr.draw_string((16.0, format!("{}", num), 0xffffffff), state_c.borrow().clone());
                                bounds.debug_draw(0xff9020ff);
                                // println!("POS1 {:?}", state_c.borrow());
                                state_c.borrow_mut().offset((el.bounds().width(), el.bounds().height()));
                                // println!("POS2 {:?}", state_c.borrow());
                                el.set_bounds(bounds);
                                // println!("set bounds {:?}", bounds);
                            }
                        })
                        .build();
                    el.handle(&Event::Render(RenderPass::Main));
                    let id = el.ui_id();
                    Some((Box::new(el) as Box<dyn UIHandler>, id))
                } else { None }
            }
        );

        layer_0.add(el_test);
        layer_0.add(el_1.build());
        // layer_0.add(Textbox::new(context().fonts().font("main").unwrap(), self.text.clone())); // "".to_string() self.text.clone()
        self.text = "".to_string();

        vec![layer_0]
        // vec![]
    }

    unsafe fn should_render(&mut self, rp: &RenderPass) -> bool {
        true
        // if rp == &RenderPass::Main {
        //     let res = self.last_fps != context().fps();
        //     res
        // } else {
        //     false
        // }
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