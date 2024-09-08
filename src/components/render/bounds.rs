use std::ops::{Add, Div, Mul, Sub};

use crate::components::render::color::ToColor;
use crate::components::render::renderer::RendererWrapped;
use crate::components::window::Window;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub trait ToBounds {
    fn to_bounds(&self) -> Bounds;
}

impl ToBounds for Bounds {
    fn to_bounds(&self) -> Bounds {
        self.clone()
    }
}

impl ToBounds for &Bounds {
    fn to_bounds(&self) -> Bounds {
        *self.clone()
    }
}

impl Sub for Bounds {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Bounds { x: self.x - other.x, y: self.y - other.y, width: self.width - other.width, height: self.height - other.height, }
    }
}

impl Add for Bounds {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Bounds { x: self.x + other.x, y: self.y + other.y, width: self.width + other.width, height: self.height + other.height, }
    }
}

impl Mul for Bounds {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Bounds { x: self.x * other.x, y: self.y * other.y, width: self.width * other.width, height: self.height * other.height, }
    }
}

impl Mul<f32> for Bounds {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Bounds { x: self.x * other, y: self.y * other, width: self.width * other, height: self.height * other, }
    }
}

impl Div for Bounds {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Bounds { x: self.x / other.x, y: self.y / other.y, width: self.width / other.width, height: self.height / other.height, }
    }
}

impl Bounds {
    pub unsafe fn draw_bounds(&self, window: &mut Window, color: impl ToColor) {
        window.renderer.draw_rect_outline(self, 1.0, color);
    }

    /// Creates a `Bounds` object from `Left, Top, Right, Bottom` parameters
    pub fn ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Bounds {
        let mut obj = Bounds::default();
        obj.set_left(left);
        obj.set_top(top);
        obj.set_right(right);
        obj.set_bottom(bottom);

        obj
    }

    /// Creates a `Bounds` object from `X, Y, Width, Height` parameters
    pub fn xywh(x: f32, y: f32, width: f32, height: f32) -> Bounds {
        let mut obj = Bounds::default();
        obj.set_x(x);
        obj.set_y(y);
        obj.set_width(width);
        obj.set_height(height);

        obj
    }

    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn width(&self) -> f32 { self.width }
    pub fn height(&self) -> f32 { self.height }
    pub fn left(&self) -> f32 { self.x }
    pub fn top(&self) -> f32 { self.y }
    pub fn right(&self) -> f32 { self.x + self.width }
    pub fn bottom(&self) -> f32 { self.y + self.height }
    pub fn center_x(&self) -> f32 { self.x + self.width / 2.0 }
    pub fn center_y(&self) -> f32 { self.y + self.height / 2.0 }

    pub fn set_x(&mut self, x: f32) { self.x = x; }
    pub fn set_y(&mut self, y: f32) { self.y = y; }
    pub fn set_width(&mut self, width: f32) { self.width = width; }
    pub fn set_height(&mut self, height: f32) { self.height = height; }
    pub fn set_right(&mut self, right: f32) { self.width = right-self.x; }
    pub fn set_bottom(&mut self, bottom: f32) { self.height = bottom-self.y; }
    pub fn set_left(&mut self, left: f32) {
        self.width += self.x - left; // Increases width, since setting the left of the bounds means the right shouldn't move
        self.x = left;
    }
    pub fn set_top(&mut self, top: f32) {
        self.height += self.y - top; // Increases height, since setting the top of the bounds means the bottom shouldn't move
        self.y = top;
    }
}