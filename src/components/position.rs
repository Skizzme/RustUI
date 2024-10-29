use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub};
use crate::components::bounds::Bounds;

#[derive(Clone, Debug, Copy,)]
pub struct Pos {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.x.to_be_bytes());
        state.write(&self.y.to_be_bytes());
    }
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self { Pos { x, y } }

    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn xy(&self) -> (f32,f32) { (self.x, self.y) }

    pub fn intersects(&self, bounds: &Bounds) -> bool {
        self.x >= bounds.left() && self.y >= bounds.top() && self.x <= bounds.right() && self.y <= bounds.bottom()
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Pos {
    type Output = Pos;
    fn sub(self, rhs: Self) -> Self::Output {
        Pos { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Into<Pos> for (f32, f32) {
    fn into(self) -> Pos {
        Pos {x: self.0, y: self.1 }
    }
}

impl Into<(f32, f32)> for Pos {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Into<Pos> for (f64, f64) {
    fn into(self) -> Pos {
        Pos {x: self.0 as f32, y: self.1 as f32 }
    }
}

impl Into<(f64, f64)> for Pos {
    fn into(self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}

impl Into<Pos> for Bounds {
    fn into(self) -> Pos { Pos::new(self.x(), self.y()) }
}
impl Into<Pos> for &Bounds {
    fn into(self) -> Pos { Pos::new(self.x(), self.y()) }
}
impl Into<Pos> for &mut Bounds {
    fn into(self) -> Pos { Pos::new(self.x(), self.y()) }
}

impl AddAssign<(f32, f32)> for Pos {
    fn add_assign(&mut self, rhs: (f32, f32)) {
        *self = Pos {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl MulAssign<(f32, f32)> for Pos {
    fn mul_assign(&mut self, rhs: (f32, f32)) {
        *self = Pos { x: self.x * rhs.0, y: self.y * rhs.1 }
    }
}

impl DivAssign<(f32, f32)> for Pos {
    fn div_assign(&mut self, rhs: (f32, f32)) {
        *self = Pos { x: self.x / rhs.0, y: self.y / rhs.1 }
    }
}