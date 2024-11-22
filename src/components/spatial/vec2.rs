use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub};
use num_traits::NumCast;
use crate::components::spatial::vec4::Vec4;

#[derive(Clone, Debug, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.x.to_be_bytes());
        state.write(&self.y.to_be_bytes());
    }
}

impl Eq for Vec2 {}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Vec2 { x, y } }

    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn xy(&self) -> (f32,f32) { (self.x, self.y) }

    pub fn intersects(&self, vec4: &Vec4) -> bool {
        self.x >= vec4.left() && self.y >= vec4.top() && self.x <= vec4.right() && self.y <= vec4.bottom()
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<A: NumCast, B: NumCast> Add<(A, B)> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: (A, B)) -> Self::Output {
        Vec2 { x: self.x + rhs.0.to_f32().unwrap_or(0.0), y: self.y + rhs.1.to_f32().unwrap_or(0.0) }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

// impl Into<Vec2> for (f32, f32) {
//     fn into(self) -> Vec2 {
//         Vec2 {x: self.0, y: self.1 }
//     }
// }

impl Into<(f32, f32)> for Vec2 {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl<A: NumCast, B: NumCast> Into<Vec2> for (A, B) {
    fn into(self) -> Vec2 {
        Vec2 {x: self.0.to_f32().unwrap_or(0.0), y: self.1.to_f32().unwrap_or(0.0) }
    }
}

impl Into<(f64, f64)> for Vec2 {
    fn into(self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}

impl Into<Vec2> for Vec4 {
    fn into(self) -> Vec2 { Vec2::new(self.x(), self.y()) }
}
impl Into<Vec2> for &Vec4 {
    fn into(self) -> Vec2 { Vec2::new(self.x(), self.y()) }
}
impl Into<Vec2> for &mut Vec4 {
    fn into(self) -> Vec2 { Vec2::new(self.x(), self.y()) }
}

impl AddAssign<(f32, f32)> for Vec2 {
    fn add_assign(&mut self, rhs: (f32, f32)) {
        *self = Vec2 {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl MulAssign<(f32, f32)> for Vec2 {
    fn mul_assign(&mut self, rhs: (f32, f32)) {
        *self = Vec2 { x: self.x * rhs.0, y: self.y * rhs.1 }
    }
}

impl DivAssign<(f32, f32)> for Vec2 {
    fn div_assign(&mut self, rhs: (f32, f32)) {
        *self = Vec2 { x: self.x / rhs.0, y: self.y / rhs.1 }
    }
}