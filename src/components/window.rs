use crate::components::position::Pos;

pub struct Window {
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) mouse_pos: Pos,
}

impl Window {
    pub fn new(width: i32, height: i32) -> Window {
        Window {
            width,
            height,
            mouse_pos: Pos::new(0,0),
        }
    }
}