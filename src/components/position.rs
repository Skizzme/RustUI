use std::ops::{Add, Sub};

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