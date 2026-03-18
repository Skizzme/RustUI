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
use ferrum::components::framework::element::ui_traits::{UIHandler, UIHandlerRef, UIIdentifier};
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
use ferrum::{element, fb_mask, register_anims, text};


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
            // context().renderer().draw_rect(context().window().bounds(), 0xff181818);

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

            // fr.draw_string((20., "there should be a tab right\there", 0xffffffff), (10., 10.,));

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
            // gl::Finish();
            // let et = st.elapsed();
            // println!("drawed {:?}", et);
            // context().renderer().draw_rect(Vec4::xywh(500., 100., 2., fr_d.height()), 0xffffffff);
            //
            // let (_, bounds) = context().fonts().font("main").unwrap().draw_string((32., "32 size", 0xffffffff), (200., 200.));
            // context().renderer().draw_rect(bounds, (1., 0.25, 0., 1.));
            //
            // context().renderer().draw_rect(Vec4::ltrb(10.0, 10.0, 200.0, 200.0), 0x90ff0000);
            fr.draw_string(text!(AlignH(Right), (20.0, format!("{:?}", context().fps()), 0xffffffff)), context().window().bounds().wh().offset_new((-4., -20.)));
            // let formated: Text = (self.t_size.borrow().value(), "context().fonts().set_font_bytes(\"main\", include_bytes!(\"assets/fonts/JetBrainsMono-Medium.ttf\").to_vec());", 0xffffffff).into();
            // let pos = 0; //
            // fr.draw_string(formated, (pos, 300.0));
            self.last_fps = context().fps();
            // Finish();
            // let st = Instant::now();
            self.test_shape.render();

            match &self.txt {
                None => {}
                Some(v) => {
                    let text  = v.borrow().get_text();
                    println!("{}", text);
                }
            }

            // context().renderer().draw_rounded_rect(Vec4::xywh(100., 200., 20., 20.), 0., 0xff206090);
            // Finish();
            // let et = Instant::now();
            // println!("{:?}", et - st);
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
                    // let mouse = context().window().mouse();
                    // let st = Instant::now();
                    // let (end_pos, vec4) = context().fonts().font("main").unwrap().draw_string(t_test_c.lock().clone(), el.bounds());
                    // el.bounds().set_width(vec4.width());
                    // el.bounds().set_height(vec4.height());
                    // let hovering = el.hovering();
                    // el.bounds().debug_draw(if hovering { 0xff10ff10 } else { 0xffffffff });

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
                        // context().renderer().draw_rect(*el.bounds(), 0xffffffff);
                        // context().renderer().draw_rect(padded, 0xff10ff10);
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
                                let render_data = fr.draw_string((16.0, format!("{}", num), 0xffffffff), state_c.borrow().clone());

                                let bounds = render_data.bounds();
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

        let hover_anim: AnimationRef = Animation::zero_ref();
        let drop_anim: AnimationRef = Animation::zero_ref();
        let mut bg_rect = Rect::new(Vec4::xywh(0, 0, 0, 0), solid(0xffffffff));
        let mut mask = FramebufferMask::new();
        let mut m_rect = Rect::new(Vec4::xywh(5,5,30,30), solid(0xffffffff));

        let mut element =
            ElementBuilder::new()
                .register_animations(vec![hover_anim.clone(), drop_anim.clone()])
                .handler(move |el, e|{
                    let hover_anim = hover_anim.clone();
                    let drop_anim = drop_anim.clone();

                    if e.is_render(RenderPass::Main) {

                        fb_mask!(
                            mask_obj: &mut mask,
                            mask: {
                                m_rect.render();
                            },
                            draw: {
                                bg_rect.render();
                            }
                        );
                        // mask.begin_mask();
                        // mask.end_mask();
                        // mask.begin_draw();
                        // bg_rect.render();
                        // mask.end_draw();
                        //
                        // mask.render();
                        // context().renderer().draw_rounded_rect(el.bounds(), 5., (0.1 * c_mult, 0.1 * c_mult, 0.1 * c_mult, 1.));
                    }
                    if e.is_prerender() {
                        hover_anim.borrow_mut().animate_to( if el.hovering() { 1.5 } else { 1.0 }, 5., Easing::Sin);
                        drop_anim.borrow_mut().animate(4.0, Easing::Progressive(1.0));
                        el.bounds().set_height(90.+90.*drop_anim.borrow().value());
                        let c_mult = hover_anim.borrow().value();
                        let color = (0.1, 0.1, 0.1, 1.).to_color().mult_rgb(1.);
                        bg_rect.set_colors(solid(color));
                        bg_rect.set_radius((hover_anim.borrow().value() - 1.) * 2. * 15.);
                        bg_rect.set_bounds(el.bounds());
                        m_rect.bounds().current_mut().set_pos(bg_rect.bounds().current().pos() / 2. + (30., 30.));

                        bg_rect.pre_render();
                        m_rect.pre_render();
                    }
                    match e {
                        Event::MouseClick(button, action) => if *action == Action::Release {
                            if el.hovering() {
                                if drop_anim.borrow().target() == 1.0 {
                                    drop_anim.borrow_mut().set_target(0.0);
                                } else {
                                    drop_anim.borrow_mut().set_target(1.0);
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .bounds(Vec4::xywh(10, 10, 140, 90))
                .draggable(true)
                .build();

        layer_0.add(element);
        // layer_0.add(el_test);
        // layer_0.add(el_1.build());
        let mut layer_1 = Layer::new((32,32));
        let (ui, txt) = UIHandlerRef::new(Textbox::new("main", &"Ï".to_string()));
        layer_1.add(ui); // "".to_string() self.text.clone()
        self.txt = Some(txt);

        // layer
        self.text = "".to_string();

        // layer_0.elements().first().unwrap().

        vec![layer_0, layer_1]
        // vec![]
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

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        true
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