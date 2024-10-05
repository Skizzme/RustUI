use crate::components::context::context;
use crate::components::framework::event::Event;

pub trait ScreenTrait {
    unsafe fn event(&mut self, event: Event);
}

pub struct DefaultScreen {

}

impl ScreenTrait for DefaultScreen {
    unsafe fn event(&mut self, event: Event) {
        match event {
            Event::Render(timed) => {
                context().fonts().get_font("main").draw_string(30.0, "toast", (200.0, 100.0), 0xffffffff);

            }
            Event::MouseClick(_, _) => {}
            Event::Keyboard(_, _, _) => {}
        }
    }
}