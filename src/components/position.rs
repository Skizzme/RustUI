use std::ops::{Add, Sub};

#[derive(Clone, Debug, Copy,)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
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

impl Into<Pos> for (i32, i32) {
    fn into(self) -> Pos {
        Pos {x: self.0, y: self.1 }
    }
}

impl Into<(i32, i32)> for Pos {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}