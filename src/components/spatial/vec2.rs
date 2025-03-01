use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub};

use num_traits::{Num, NumCast, ToPrimitive};

use crate::components::spatial::vec4::Vec4;

/// Stores x and y values as f32 values
#[derive(Clone, Debug, Copy, PartialEq, Default)]
pub struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> PartialOrd<Self> for Vec2<T> where T: Eq + Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Vec2<T>
where T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        } else if self.y > other.y {
            Ordering::Greater
        } else {
            self.x.cmp(&other.y)
        }
    }
}

impl<T> Eq for Vec2<T> where T: Eq {}

impl<T> Vec2<T>
where T: Num + Clone + AddAssign
{
    pub fn zero() -> Self { Vec2 { x: T::zero(), y: T::zero() } }
    pub fn new(x: T, y: T) -> Self { Vec2 { x, y } }

    pub fn x(&self) -> T { self.x.clone() }
    pub fn y(&self) -> T { self.y.clone() }
    pub fn xy(&self) -> (T,T) { (self.x.clone(), self.y.clone()) }

    pub fn set_x(&mut self, x: T) { self.x = x; }
    pub fn set_y(&mut self, y: T) { self.y = y; }

    /// Adds the components of both [`Vec2`]
    pub fn offset(&mut self, vec2: impl Into<Vec2<T>>) {
        let vec2 = vec2.into();
        self.x += vec2.x;
        self.y += vec2.y;
    }

    /// Returns if this [`Vec2`] is inside the bounds of the [`Vec4`]
    pub fn intersects(&self, vec4: &Vec4) -> bool
    where T: PartialOrd<f32>
    {
        self.x >= vec4.left() && self.y >= vec4.top() && self.x <= vec4.right() && self.y <= vec4.bottom()
    }
}

impl<T> Add for Vec2<T>
where T: Add<Output = T>
{
    type Output = Vec2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<A: NumCast, B: NumCast> Add<(A, B)> for Vec2<f32> {
    type Output = Vec2<f32>;
    fn add(self, rhs: (A, B)) -> Self::Output {
        Vec2 { x: self.x + rhs.0.to_f32().unwrap_or(0.0), y: self.y + rhs.1.to_f32().unwrap_or(0.0) }
    }
}

impl<T> Sub for Vec2<T>
where T: Sub<Output = T>,
{
    type Output = Vec2<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Into<(f32, f32)> for Vec2<f32> {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl<T: NumCast> Into<Vec2<T>> for (T, T) {
    fn into(self) -> Vec2<T> {
        Vec2 { x: self.0, y: self.1 }
    }
}

impl<T> Into<(f64, f64)> for Vec2<T>
where T: NumCast
{
    fn into(self) -> (f64, f64) {
        (self.x.to_f64().unwrap(), self.y.to_f64().unwrap())
    }
}

impl Into<Vec2<f32>> for Vec4 {
    fn into(self) -> Vec2<f32> { Vec2::new(self.x(), self.y()) }
}
impl Into<Vec2<f32>> for &Vec4 {
    fn into(self) -> Vec2<f32> { Vec2::new(self.x(), self.y()) }
}
impl Into<Vec2<f32>> for &mut Vec4 {
    fn into(self) -> Vec2<f32> { Vec2::new(self.x(), self.y()) }
}

impl AddAssign<(f32, f32)> for Vec2<f32> {
    fn add_assign(&mut self, rhs: (f32, f32)) {
        *self = Vec2 {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl MulAssign<(f32, f32)> for Vec2<f32> {
    fn mul_assign(&mut self, rhs: (f32, f32)) {
        *self = Vec2 { x: self.x * rhs.0, y: self.y * rhs.1 }
    }
}

impl DivAssign<(f32, f32)> for Vec2<f32> {
    fn div_assign(&mut self, rhs: (f32, f32)) {
        *self = Vec2 { x: self.x / rhs.0, y: self.y / rhs.1 }
    }
}