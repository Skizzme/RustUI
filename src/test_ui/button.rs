#[allow(unused)]
pub struct Button<C> where C: FnMut() {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    on_click: C,
}

impl<C> Button<C> where C: FnMut() {
    pub fn default<F>(x: f32, y: f32, width: f32, height: f32, on_click: F) -> Button<F> where F: FnMut() {
        Button {
            x,
            y,
            width,
            height,
            on_click
        }
    }
}