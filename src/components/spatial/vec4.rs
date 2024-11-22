use std::ops::{Add, Div, Mul, Sub};

use crate::components::context::context;
use crate::components::render::color::ToColor;
use crate::components::spatial::vec2::Vec2;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Vec4 {
    pub unsafe fn draw_vec4(&self, color: impl ToColor) {
        context().renderer().draw_rect_outline(self, 1.0, color);
    }

    /// Creates a `Vec4` object from `Left, Top, Right, Bottom` parameters
    pub fn ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Vec4 {
        let mut obj = Vec4::default();
        obj.set_left(left);
        obj.set_top(top);
        obj.set_right(right);
        obj.set_bottom(bottom);

        obj
    }


    /// Creates a 'Vec4' object from 2 positions. The order does not matter, the object will be created
    /// by taking the minimum and maximum of the X and Y of both positions. Negative widths and heights
    /// will never occur
    pub fn from_pos<A: Into<Vec2>, B: Into<Vec2>>(pos1: A, pos2: B) -> Vec4 {
        let pos1 = pos1.into();
        let pos2 = pos2.into();

        let mut obj = Vec4::default();

        obj.set_left(pos1.x().min(pos2.x()));
        obj.set_right(pos1.x().max(pos2.x()));
        obj.set_top(pos1.y().min(pos2.y()));
        obj.set_bottom(pos1.y().max(pos2.y()));

        obj
    }

    /// Creates a `Vec4` object from `X, Y, Width, Height` parameters
    pub fn xywh(x: f32, y: f32, width: f32, height: f32) -> Vec4 {
        let mut obj = Vec4::default();
        obj.set_x(x);
        obj.set_y(y);
        obj.set_width(width);
        obj.set_height(height);

        obj
    }

    pub fn top_left(&self) -> Vec2 { Vec2::new(self.left().min(self.right()), self.top().min(self.bottom()) ) }
    pub fn top_right(&self) -> Vec2 { Vec2::new(self.left().max(self.right()), self.top().min(self.bottom()) ) }
    pub fn bottom_left(&self) -> Vec2 { Vec2::new(self.left().min(self.right()), self.top().max(self.bottom()) ) }
    pub fn bottom_right(&self) -> Vec2 { Vec2::new(self.left().max(self.right()), self.top().max(self.bottom()) ) }

    pub fn pos(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn xy(&self) -> Vec2 { Vec2::new(self.x, self.y) }
    pub fn wh(&self) -> Vec2 { Vec2::new(self.width, self.height) }
    pub fn width(&self) -> f32 { self.width }
    pub fn height(&self) -> f32 { self.height }
    pub fn left(&self) -> f32 { self.x }
    pub fn top(&self) -> f32 { self.y }
    pub fn right(&self) -> f32 { self.x + self.width }
    pub fn bottom(&self) -> f32 { self.y + self.height }
    pub fn center_x(&self) -> f32 { self.x + self.width / 2.0 }
    pub fn center_y(&self) -> f32 { self.y + self.height / 2.0 }

    pub fn set_pos(&mut self, pos: &Vec2) {
        self.x = pos.x;
        self.y = pos.y;
    }

    pub fn set_x(&mut self, x: f32) { self.x = x; }
    pub fn set_y(&mut self, y: f32) { self.y = y; }
    pub fn set_width(&mut self, width: f32) { self.width = width; }
    pub fn set_height(&mut self, height: f32) { self.height = height; }
    pub fn set_right(&mut self, right: f32) { self.width = right-self.x; }
    pub fn set_bottom(&mut self, bottom: f32) { self.height = bottom-self.y; }
    pub fn set_left(&mut self, left: f32) {
        self.width += self.x - left; // Increases width, since setting the left of the vec4 means the right shouldn't move
        self.x = left;
    }
    pub fn set_top(&mut self, top: f32) {
        self.height += self.y - top; // Increases height, since setting the top of the vec4 means the bottom shouldn't move
        self.y = top;
    }

    pub fn expand(&mut self, value: f32) {
        self.x -= value;
        self.y -= value;
        self.width += value*2.0;
        self.height += value*2.0;
    }

    pub fn expand_to_x(&mut self, x: f32) {
        if self.x > x {
            self.set_left(x);
        } else if (self.right() < x) {
            self.set_right(x);
        }
    }

    pub fn expand_to_y(&mut self, y: f32) {
        if self.y > y {
            self.set_top(y);
        } else if (self.bottom() < y) {
            self.set_bottom(y);
        }
    }

    pub fn shrink(&mut self, other: &Vec4) -> Vec4 {
        let mut this = self.clone();
        this.set_x(this.x + other.x);
        this.set_width(this.width - other.width - other.x);
        this.set_y(this.y + other.y);
        this.set_height(this.height - other.height - other.y);
        this
    }

    pub fn offset<A: Into<Vec2>>(&mut self, amount: A) {
        let amount = amount.into();
        self.x += amount.x;
        self.y += amount.y;
    }
}

impl Into<Vec4> for &Vec4 {
    fn into(self) -> Vec4 { self.clone() }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec4 { x: self.x - other.x, y: self.y - other.y, width: self.width - other.width, height: self.height - other.height, }
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec4 { x: self.x + other.x, y: self.y + other.y, width: self.width + other.width, height: self.height + other.height, }
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec4 { x: self.x * other.x, y: self.y * other.y, width: self.width * other.width, height: self.height * other.height, }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Vec4 { x: self.x * other, y: self.y * other, width: self.width * other, height: self.height * other, }
    }
}

impl Div for Vec4 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Vec4 { x: self.x / other.x, y: self.y / other.y, width: self.width / other.width, height: self.height / other.height, }
    }
}