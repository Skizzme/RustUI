use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub};
use crate::components::bounds::Bounds;

#[derive(Clone, Debug, Copy,)]
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

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Vec2 { x, y } }

    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn xy(&self) -> (f32,f32) { (self.x, self.y) }

    pub fn intersects(&self, bounds: &Bounds) -> bool {
        self.x >= bounds.left() && self.y >= bounds.top() && self.x <= bounds.right() && self.y <= bounds.bottom()
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add<(f32, f32)> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: (f32, f32)) -> Self::Output {
        Vec2 { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Into<Vec2> for (f32, f32) {
    fn into(self) -> Vec2 {
        Vec2 {x: self.0, y: self.1 }
    }
}

impl Into<(f32, f32)> for Vec2 {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Into<Vec2> for (f64, f64) {
    fn into(self) -> Vec2 {
        Vec2 {x: self.0 as f32, y: self.1 as f32 }
    }
}

impl Into<(f64, f64)> for Vec2 {
    fn into(self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}

impl Into<Vec2> for Bounds {
    fn into(self) -> Vec2 { Vec2::new(self.x(), self.y()) }
}
impl Into<Vec2> for &Bounds {
    fn into(self) -> Vec2 { Vec2::new(self.x(), self.y()) }
}
impl Into<Vec2> for &mut Bounds {
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