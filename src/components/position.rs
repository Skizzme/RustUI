use std::ops::{Add, AddAssign, Sub};
use crate::components::bounds::Bounds;

#[derive(Clone, Debug, Copy,)]
pub struct Pos {
    pub(crate) x: f32,
    pub(crate) y: f32,
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