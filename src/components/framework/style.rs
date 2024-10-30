use std::collections::HashMap;
use crate::components::bounds::Bounds;
use crate::components::position::Pos;
use crate::components::render::color::{Color, ToColor};

#[derive(Debug, Clone)]
pub enum Style {
    Color(Color),
    Bounds(Bounds),
    Pos(Pos),
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

    pub fn set_bounds(&mut self, key: impl ToString, value: Bounds) {
        self.set(key, Style::Bounds(value));
    }

    pub fn set_pos(&mut self, key: impl ToString, value: Pos) {
        self.set(key, Style::Pos(value));
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

    pub fn get_bounds(&self, key: impl ToString) -> Bounds {
        match self.get(key) {
            Style::Bounds(b) => b.clone(),
            _ => Bounds::xywh(0.0,0.0,0.0,0.0),
        }
    }

    pub fn get_pos(&self, key: impl ToString) -> Pos {
        match self.get(key) {
            Style::Pos(p) => p.clone(),
            _ => Pos::new(0.0,0.0),
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