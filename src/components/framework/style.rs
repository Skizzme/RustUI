use std::collections::HashMap;

use crate::components::spatial::vec4::Vec4;
use crate::components::spatial::vec2::Vec2;
use crate::components::render::color::{Color, ToColor};

#[derive(Debug, Clone)]
pub enum Style {
    Color(Color),
    Vec4(Vec4),
    Vec2(Vec2),
    Number(f32),
    String(String),
    Bool(bool),
    None,
}

#[derive(Debug, Clone)]
pub struct StyleRegistry {
    styles: HashMap<String, Style>,
}

impl StyleRegistry {
    pub fn new() -> Self {
        StyleRegistry {
            styles: HashMap::new()
        }
    }

    pub fn set(&mut self, key: impl ToString, value: Style) {
        let key = key.to_string();
        self.styles.insert(key, value);
    }

    pub fn set_vec4(&mut self, key: impl ToString, value: Vec4) {
        self.set(key, Style::Vec4(value));
    }

    pub fn set_pos(&mut self, key: impl ToString, value: Vec2) {
        self.set(key, Style::Vec2(value));
    }

    pub fn set_color(&mut self, key: impl ToString, value: impl ToColor) {
        self.set(key, Style::Color(value.to_color()));
    }

    pub fn set_number(&mut self, key: impl ToString, value: f32) {
        self.set(key, Style::Number(value));
    }

    pub fn set_string(&mut self, key: impl ToString, value: String) {
        self.set(key, Style::String(value));
    }

    pub fn set_bool(&mut self, key: impl ToString, value: bool) {
        self.set(key, Style::Bool(value));
    }

    pub fn get(&self, key: impl ToString) -> &Style {
        let key = key.to_string();
        self.styles.get(&key).unwrap_or(&Style::None)
    }

    pub fn get_color(&self, key: impl ToString) -> Color {
        match self.get(key) {
            Style::Color(c) => c.clone(),
            _ => Color::from_u32(0xffffffff),
        }
    }

    pub fn get_number(&self, key: impl ToString) -> f32 {
        match self.get(key) {
            Style::Number(v) => *v,
            _ => 0.0f32
        }
    }

    pub fn get_vec4(&self, key: impl ToString) -> Vec4 {
        match self.get(key) {
            Style::Vec4(b) => b.clone(),
            _ => Vec4::xywh(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn get_pos(&self, key: impl ToString) -> Vec2 {
        match self.get(key) {
            Style::Vec2(p) => p.clone(),
            _ => Vec2::new(0.0, 0.0),
        }
    }

    pub fn get_bool(&self, key: impl ToString) -> bool {
        match self.get(key) {
            Style::Bool(v) => *v,
            _ => false
        }
    }

    pub fn get_string(&self, key: impl ToString) -> String {
        match self.get(key) {
            Style::String(v) => v.clone(),
            _ => "".to_string()
        }
    }
}

pub trait StyleTrait {
    fn styles(&mut self) -> &mut StyleRegistry;
}