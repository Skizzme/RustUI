use glfw::MouseButton;
use crate::components::bounds::Bounds;
use crate::components::context::context;
use crate::components::framework::element::Element;
use crate::components::framework::event::Event;

pub trait ScreenTrait {
    unsafe fn event(&mut self, event: &Event);
    unsafe fn register_elements(&mut self) -> Vec<Element>;
}

pub struct DefaultScreen {
    pub text: String,
}

impl ScreenTrait for DefaultScreen {
    unsafe fn event(&mut self, event: &Event) {
        match event {
            Event::Render(_) => {
                context().fonts().get_font("main").draw_string(30.0, &self.text, (200.0, 100.0), 0xffffffff);
                context().fonts().get_font("main").draw_string(30.0, &self.text, (200.0, 300.0), 0xffffffff);
            }
            Event::MouseClick(_, _) => {}
            Event::Keyboard(_, _, _) => {}
            _ => {}
        }
    }

    unsafe fn register_elements(&mut self) -> Vec<Element> {
        vec![
            Element::new(Bounds::xywh(5.0, 100.0, 100.0, 100.0), true, |el, event| {
                match event {
                    Event::Render(_) => {
                        let mouse = context().window().mouse();
                        let (width, height) = context().fonts().get_font("main").draw_string(40.0, format!("{:?}", mouse.pos()), el.bounds(), 0xffffffff);
                        el.bounds().set_width(width);
                        el.bounds().set_height(height);
                        let hovering = el.hovering();
                        el.bounds().draw_bounds(if hovering { 0xff10ff10 } else { 0xffffffff });
                    }
                    _ => {}
                }
            } ),
            Element::new(Bounds::xywh(200.0, 400.0, 100.0, 100.0), true, |el, event| {
                match event {
                    Event::Render(_) => {
                        let mouse = context().window().mouse();
                        let (width, height) = context().fonts().get_font("main").draw_string(40.0, format!("{:?}", mouse.is_pressed(MouseButton::Button1)), el.bounds(), 0xffffffff);
                        el.bounds().set_width(width);
                        el.bounds().set_height(height);
                        let hovering = el.hovering();
                        el.bounds().draw_bounds(if hovering { 0xff10ff10 } else { 0xffffffff });
                    }
                    _ => {}
                }
            } )
        ]
    }
}