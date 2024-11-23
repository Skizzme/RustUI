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

use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Instant, UNIX_EPOCH};

use glfw::{Action, Key, WindowHint};
use parking_lot::{Mutex, MutexGuard};
use winapi::um::wincon::FreeConsole;

use RustUI::components::spatial::vec4::Vec4;
use RustUI::components::context::{context, ContextBuilder};
use RustUI::components::editor::Textbox;
use RustUI::components::framework::animation::{Animation, AnimationRef, AnimationRegistry, AnimationType};
use RustUI::components::framework::element::{ElementBuilder, MultiElement, UIHandler, UIIdentifier};
use RustUI::components::framework::event::{Event, RenderPass};
use RustUI::components::framework::layer::Layer;
use RustUI::components::framework::screen::ScreenTrait;
use RustUI::components::spatial::vec2::Vec2;
use RustUI::components::render::font::Font;
use RustUI::components::render::font::format::FormattedText;
use RustUI::components::render::font::renderer::{FontRenderer};
use RustUI::components::render::mask::FramebufferMask;
use RustUI::components::render::renderer::shader_file;
use RustUI::components::render::stack::{GlState, State};
use RustUI::components::wrapper::shader::Shader;
use RustUI::components::wrapper::texture::Texture;

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
    previous_pos: Vec2,
    previous_tex: Arc<Mutex<String>>,
    t_size: AnimationRef,
    last_fps: u32,
    t_text: Arc<Mutex<FormattedText>>,
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
        context().fonts().set_font_bytes("main", include_bytes!("assets/fonts/JetBrainsMono-Medium.ttf").to_vec());
        // context().fonts().load_font("main", true);

        let v = vec![Test::new(1), Test::new(2), Test::new(3), Test::new(4),Test::new(1000)];
        let test_vec = Arc::new(Mutex::new(v));
        TestScreen {
            text: t,
            fr: context().fonts().renderer("main"),
            previous_pos: Vec2::new(0.0, 0.0),
            previous_tex: Arc::new(Mutex::new("".to_string())),
            t_size: Rc::new(RefCell::new(Animation::zero())),
            last_fps: 0,
            t_text: Arc::new(Mutex::new(FormattedText::new())),
            mask: FramebufferMask::new(),
            t_shader: Shader::new(shader_file("shaders/vertex.glsl"), shader_file("shaders/test.frag")),
            items: test_vec,
        }
    }
}

impl ScreenTrait for TestScreen {
    unsafe fn handle(&mut self, event: &Event) {
        match event {
            Event::PreRender => {
                self.t_size.borrow_mut().animate(4f32, AnimationType::Sin);
            }
            Event::Keyboard(key, action, _) => {
                if action != &Action::Press {
                    return;
                }
                let len = self.items.lock().len();
                match key {
                    Key::Backspace => {
                        self.items.lock().pop();
                    }
                    _ => {
                        self.items.lock().push(Test::new(len as i32));
                    }
                }
            }
            Event::Scroll(_, y) => {
                let current = self.t_size.borrow().target();
                self.t_size.borrow_mut().set_target(*y + current);
                // println!("SCROLL {}", y);
            }
            Event::Render(pass) => {
                if pass != &RenderPass::Main {
                    return;
                }
                // match context().fonts().font("main") {
                //     None => {}
                //     Some(font) => {
                //         self.t_shader.bind();
                //         let tex = font.atlas_tex.as_ref().unwrap();
                //         tex.bind();
                //         // context().window().width()
                //         context().renderer().draw_texture_rect(Vec4::xywh(0.0, 0.0, tex.width as f32, tex.height as f32), 0xffffffff);
                //         Texture::unbind();
                //         Shader::unbind();
                //     }
                // }

                context().renderer().draw_rect(Vec4::ltrb(10.0, 10.0, 200.0, 200.0), 0x90ff0000);
                // self.fr.draw_string((30.0, "something", 0xffffffff), (0.0, 0.0));
                self.fr.draw_string((30.0, format!("{:?}", context().fps()), 0xffffffff), (200.0, 100.0));
                self.last_fps = context().fps();

                // self.mask.begin_mask();
                // context().renderer().draw_circle(200.0, 200.0, 175.0, 0xffffffff);
                // self.mask.end_mask();
                // self.mask.begin_draw();
                // self.fr.draw_string((self.t_size.borrow().value() * 1f32, &self.text, 0x90ffffff), (10.0, 10.0));
                // self.mask.end_draw();
                // self.mask.render();
            }
            Event::PostRender => {
                self.previous_pos = *context().window().mouse().pos();
            }
            Event::MouseClick(_, _) => {}
            Event::Keyboard(_, _, _) => {}
            _ => {}
        }
    }

    unsafe fn init(&mut self) -> Vec<Layer> {
        context().framework().screen_animations().register(self.t_size.clone());
        let mut layer_0 = Layer::new();
        let tex_cl1 = self.previous_tex.clone();
        let tex_cl2 = self.previous_tex.clone();
        let t_test_c = self.t_text.clone();
        let el_1 = ElementBuilder::new()
            .vec4(Vec4::xywh(5.0, 100.0, 100.0, 100.0))
            .draggable(true)
            .handler(move |el, event| {
                match event {
                    Event::MousePos(x, y) => {
                        let st = Instant::now();
                        let items: Vec<FormattedText> = vec![
                            (36.0, "before", 0xffffff90).into(),
                            (context().window().mouse().pos().x() / 400.0 * 40.0, format!("&ff2030ff{} {}", x, y), 0xffffffff).into(),
                            (36.0, "after ", 0xff90ffff).into(),
                            (20.0, format!("{}", UNIX_EPOCH.elapsed().unwrap().as_secs_f64()), 0xffff2020).into()
                        ];
                        let t: FormattedText = items.into();
                        *t_test_c.lock() = t;
                    }
                    Event::Render(pass) => {
                        if pass != &RenderPass::Main {
                            return;
                        }
                        let mouse = context().window().mouse();
                        // context().renderer().draw_rect(*el.vec4(), 0xff00ff00);
                        let st = Instant::now();
                        let (end_pos, vec4) = context().fonts().renderer("main").draw_string(t_test_c.lock().clone(), el.bounds());
                        // println!("{:?}", st.elapsed());
                        el.bounds().set_width(vec4.width());
                        el.bounds().set_height(vec4.height());
                        let hovering = el.hovering();
                        el.bounds().draw_vec4(if hovering { 0xff10ff10 } else { 0xffffffff });

                        *tex_cl1.lock() = format!("{:?}", context().window().mouse().pos()).to_string();
                    }
                    _ => {}
                }
            })
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
            .vec4(Vec4::xywh(0.0, 0.0, 40.0, 40.0))
            .draggable(false)
            .handler(|el, event| {
                match event {
                    Event::Render(pass) => {
                        // context().renderer().draw_rect(*el.vec4(), 0xff90ff20);z
                        // let hovering = el.hovering();
                        if pass == &RenderPass::Bloom {
                            let mut shrunk = el.bounds().clone();
                            shrunk.expand(-10.0);
                            context().renderer().draw_rect(*el.bounds(), 0xffffffff);
                            context().renderer().draw_rect(shrunk, 0xff10ff10);
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
                }
            })
            .should_render(|_, _| {
                // println!("c el check");
                false
            });

        // let el_1 = el_1.child(el_1_c.build());

        let test_vec_1 = self.items.clone();
        let test_vec_2 = self.items.clone();
        let el_test = MultiElement::new(
            move |mut inner| {
                let mut lock = test_vec_2.lock();
                let mut state = 0;
                for item in lock.iter_mut() {
                    inner(&mut state, item);
                    state += 1;
                }
            },
            move |exists, state, item| {
                let state_c = *state;
                let num = item.v;
                (*state) += 1;
                let res = if !exists {
                    Some(Box::new(ElementBuilder::new()
                        .handler(move |el, e| {
                            match e {
                                Event::Render(pass) => {
                                    if pass != &RenderPass::Main {
                                        return;
                                    }
                                    let mut fr = context().fonts().renderer("main");
                                    fr.draw_string((16.0, format!("{}", num), 0xffffffff), (0, state_c * 20));
                                }
                                _ => {}
                            }
                        }).build()) as Box<dyn UIHandler>)
                } else { None };
                res
            }
        );

        layer_0.add(el_test);
        layer_0.add(el_1.build());
        // layer_0.add(Textbox::new(context().fonts().renderer("main"), self.text.clone())); // "".to_string() self.text.clone()
        self.text = "".to_string();

        vec![layer_0]
    }

    unsafe fn should_render(&mut self, rp: &RenderPass) -> bool {
        // true
        if rp == &RenderPass::Main {
            let res = self.last_fps != context().fps();
            res
        } else {
            false
        }
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