use crate::gl_binds::gl30::Color4f;

/// A struct to convert a color to all necessary forms
///
/// All values are 0 to 1
#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Color {
    /// The constructor to create a color from a u32, like 0xff909090
    pub fn from_u32(color: u32) -> Color {
        Color {
            red: (color >> 16 & 255) as f32 / 255f32,
            green: (color >> 8 & 255) as f32 / 255f32,
            blue: (color & 255) as f32 / 255f32,
            alpha: (color >> 24 & 255) as f32 / 255f32,
        }
    }

    /// The constructor to create a color from 0-255 u8 values
    pub fn from_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red: (red as f32/255f32).clamp(0.0, 1.0),
            green: (green as f32/255f32).clamp(0.0, 1.0),
            blue: (blue as f32/255f32).clamp(0.0, 1.0),
            alpha: (alpha as f32/255f32).clamp(0.0, 1.0),
        }
    }

    /// The constructor to create a color for 0 - 1 f32 values
    pub fn from_f32(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
            alpha: alpha.clamp(0.0, 1.0),
        }
    }

    pub fn red(&self) -> f32 {
        self.red
    }
    pub fn green(&self) -> f32 {
        self.green
    }
    pub fn blue(&self) -> f32 {
        self.blue
    }
    pub fn alpha(&self) -> f32 {
        self.alpha
    }
    pub fn rgba(&self) -> Vec<f32> {
        vec![self.red, self.green, self.blue, self.alpha]
    }
    pub fn rgba_u8(&self) -> Vec<u8> {
        vec![(self.red*255.0).round() as u8, (self.green*255.0).round() as u8, (self.blue*255.0).round() as u8, (self.alpha*255.0).round() as u8]
    }
    pub fn rgba_u32(&self) -> u32 {
        (((self.alpha * 255f32).round() as u32) << 24)
            | (((self.red * 255f32).round() as u32) << 16)
            | (((self.green * 255f32).round() as u32) << 8)
            | ((self.blue * 255f32).round() as u32)
    }

    pub fn set_red_f32(&mut self, red: f32) { self.red = red; }
    pub fn set_green_f32(&mut self, green: f32) { self.green = green; }
    pub fn set_blue_f32(&mut self, blue: f32) { self.blue = blue; }
    pub fn set_alpha_f32(&mut self, alpha: f32) { self.alpha = alpha; }
    pub fn set_red_u8(&mut self, red: u8) { self.red = (red as f32/255f32).clamp(0.0, 1.0); }
    pub fn set_green_u8(&mut self, green: u8) { self.green = (green as f32/255f32).clamp(0.0, 1.0); }
    pub fn set_blue_u8(&mut self, blue: u8) { self.blue = (blue as f32/255f32).clamp(0.0, 1.0); }
    pub fn set_alpha_u8(&mut self, alpha: u8) { self.alpha = (alpha as f32/255f32).clamp(0.0, 1.0); }
    pub fn set_color_u32(&mut self, color: u32) {
        self.alpha = (color >> 24 & 255) as f32 / 255f32;
        self.red = (color >> 16 & 255) as f32 / 255f32;
        self.green = (color >> 8 & 255) as f32 / 255f32;
        self.blue = (color & 255) as f32 / 255f32;
    }

    pub unsafe fn apply(&self) {
        Color4f(self.red, self.green, self.blue, self.alpha);
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