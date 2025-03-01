use std::hash::{Hash, Hasher};
use std::ops::Mul;

use crate::gl_binds::gl30::Color4f;

/// A struct to convert a color to all necessary forms
///
/// All values are 0 to 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.r.to_be_bytes());
        state.write(&self.g.to_be_bytes());
        state.write(&self.b.to_be_bytes());
        state.write(&self.a.to_be_bytes());
    }
}

impl Color {
    /// The constructor to create a color from a u32, like 0xff909090
    pub fn from_u32(color: u32) -> Color {
        Color {
            r: (color >> 16 & 255) as f32 / 255f32,
            g: (color >> 8 & 255) as f32 / 255f32,
            b: (color & 255) as f32 / 255f32,
            a: (color >> 24 & 255) as f32 / 255f32,
        }
    }

    /// The constructor to create a color from 0-255 u8 values
    pub fn from_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            r: (red as f32/255f32).clamp(0.0, 1.0),
            g: (green as f32/255f32).clamp(0.0, 1.0),
            b: (blue as f32/255f32).clamp(0.0, 1.0),
            a: (alpha as f32/255f32).clamp(0.0, 1.0),
        }
    }

    /// The constructor to create a color for 0 - 1 f32 values
    pub fn from_f32(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            r: red.clamp(0.0, 1.0),
            g: green.clamp(0.0, 1.0),
            b: blue.clamp(0.0, 1.0),
            a: alpha.clamp(0.0, 1.0),
        }
    }

    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
        Color {
            r: ((((hue * 6.0 - 3.0).abs() - 1.0).clamp(0.0, 1.0) - 1.0) * saturation + 1.0) * value,
            g: (((2.0 - (hue * 6.0 - 2.0).abs()).clamp(0.0, 1.0) - 1.0) * saturation + 1.0) * value,
            b: (((2.0 - (hue * 6.0 - 4.0).abs()).clamp(0.0, 1.0) - 1.0) * saturation + 1.0) * value,
            a: 1.0,
        }
    }

    pub fn red(&self) -> f32 {
        self.r
    }
    pub fn green(&self) -> f32 {
        self.g
    }
    pub fn blue(&self) -> f32 {
        self.b
    }
    pub fn alpha(&self) -> f32 {
        self.a
    }
    pub fn rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    pub fn rgba_u8(&self) -> Vec<u8> {
        vec![(self.r *255.0).round() as u8, (self.g *255.0).round() as u8, (self.b *255.0).round() as u8, (self.a *255.0).round() as u8]
    }
    pub fn rgba_u32(&self) -> u32 {
        (((self.a * 255f32).round() as u32) << 24)
            | (((self.r * 255f32).round() as u32) << 16)
            | (((self.g * 255f32).round() as u32) << 8)
            | ((self.b * 255f32).round() as u32)
    }

    pub fn set_red_f32(mut self, red: f32) -> Color { self.r = red; self }
    pub fn set_green_f32(mut self, green: f32) -> Color { self.g = green; self }
    pub fn set_blue_f32(mut self, blue: f32) -> Color { self.b = blue; self }
    pub fn set_alpha_f32(mut self, alpha: f32) -> Color { self.a = alpha; self }
    pub fn set_red_u8(mut self, red: u8) -> Color { self.r = (red as f32/255f32).clamp(0.0, 1.0); self }
    pub fn set_green_u8(mut self, green: u8) -> Color { self.g = (green as f32/255f32).clamp(0.0, 1.0); self }
    pub fn set_blue_u8(mut self, blue: u8) -> Color { self.b = (blue as f32/255f32).clamp(0.0, 1.0); self }
    pub fn set_alpha_u8(mut self, alpha: u8) -> Color { self.a = (alpha as f32/255f32).clamp(0.0, 1.0); self }
    pub fn set_color_u32(mut self, color: u32) -> Color {
        self.a = (color >> 24 & 255) as f32 / 255f32;
        self.r = (color >> 16 & 255) as f32 / 255f32;
        self.g = (color >> 8 & 255) as f32 / 255f32;
        self.b = (color & 255) as f32 / 255f32;
        self
    }

    pub fn mult_rgb(self, mult: f32) -> Color {
        Color {
            r: self.r * mult,
            g: self.g * mult,
            b: self.b * mult,
            a: self.a,
        }
    }

    pub unsafe fn apply(&self) {
        Color4f(self.r, self.g, self.b, self.a);
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

pub trait ToColor {
    fn to_color(&self) -> Color;
    unsafe fn apply_color(&self) {
        self.to_color().apply();
    }
}

impl ToColor for Color {
    fn to_color(&self) -> Color {
        self.clone()
    }
}

impl ToColor for u32 {
    fn to_color(&self) -> Color {
        Color::from_u32(*self)
    }
}

impl ToColor for (f32,f32,f32,f32) {
    fn to_color(&self) -> Color {
        Color::from_f32(self.0, self.1, self.2, self.3)
    }
}

impl ToColor for (u8,u8,u8,u8) {
    fn to_color(&self) -> Color {
        Color::from_u8(self.0, self.1, self.2, self.3)
    }
}