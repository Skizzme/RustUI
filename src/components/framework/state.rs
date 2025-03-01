use std::collections::HashMap;
use crate::components::framework::changing::Changing;

use crate::components::render::color::{Color, ToColor};
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Color(Color),
    Vec4(Vec4),
    Vec2(Vec2<f32>),
    Number(f32),
    String(String),
    Bool(bool),
    None,
}

pub trait StateRegistry {
    fn set(&mut self, key: impl ToString, value: State);

    fn get(&self, key: impl ToString) -> &State;


    fn set_vec4(&mut self, key: impl ToString, value: Vec4) {
        self.set(key, State::Vec4(value));
    }

    fn set_pos(&mut self, key: impl ToString, value: Vec2<f32>) {
        self.set(key, State::Vec2(value));
    }

    fn set_color(&mut self, key: impl ToString, value: impl ToColor) {
        self.set(key, State::Color(value.to_color()));
    }

    fn set_number(&mut self, key: impl ToString, value: f32) {
        self.set(key, State::Number(value));
    }

    fn set_string(&mut self, key: impl ToString, value: String) {
        self.set(key, State::String(value));
    }

    fn set_bool(&mut self, key: impl ToString, value: bool) {
        self.set(key, State::Bool(value));
    }

    fn get_color(&self, key: impl ToString) -> Color {
        match self.get(key) {
            State::Color(c) => c.clone(),
            _ => Color::from_u32(0xffffffff),
        }
    }

    fn get_number(&self, key: impl ToString) -> f32 {
        match self.get(key) {
            State::Number(v) => *v,
            _ => 0.0f32
        }
    }

    fn get_vec4(&self, key: impl ToString) -> Vec4 {
        match self.get(key) {
            State::Vec4(b) => b.clone(),
            _ => Vec4::xywh(0.0, 0.0, 0.0, 0.0),
        }
    }

    fn get_pos(&self, key: impl ToString) -> Vec2<f32> {
        match self.get(key) {
            State::Vec2(p) => p.clone(),
            _ => Vec2::new(0.0, 0.0),
        }
    }

    fn get_bool(&self, key: impl ToString) -> bool {
        match self.get(key) {
            State::Bool(v) => *v,
            _ => false
        }
    }

    fn get_string(&self, key: impl ToString) -> String {
        match self.get(key) {
            State::String(v) => v.clone(),
            _ => "".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnchangingRegistry {
    states: HashMap<String, State>,
}

impl UnchangingRegistry {
    pub fn new() -> Self {
        UnchangingRegistry {
            states: HashMap::new()
        }
    }
}

impl StateRegistry for UnchangingRegistry {
    fn set(&mut self, key: impl ToString, value: State) {
        let key = key.to_string();
        self.states.insert(key, value);
    }

    fn get(&self, key: impl ToString) -> &State {
        let key = key.to_string();
        self.states.get(&key).unwrap_or(&State::None)
    }
}

#[derive(Debug)]
pub struct ChangingRegistry {
    states: HashMap<String, Changing<State>>,
    sub_registries: HashMap<String, ChangingRegistry>,
}

impl ChangingRegistry {
    pub fn new() -> Self {
        ChangingRegistry {
            states: HashMap::new(),
            sub_registries: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        for state in self.states.values_mut() {
            if state.changed() {
                state.update()
            }
        }
        for (_, mut sub) in &mut self.sub_registries {
            sub.update();
        }
    }

    pub fn changed_all(&self) -> bool {
        for state in self.states.values() {
            if state.changed() {
                return true;
            }
        }
        false
    }

    pub fn changed(&self, key: impl ToString) -> bool {
        let key = key.to_string();
        if let Some(state) = self.states.get(&key) {
            return state.changed();
        }
        false
    }

    pub fn sub(&mut self, key: impl ToString) -> &mut ChangingRegistry {
        let key = key.to_string();
        if !self.sub_registries.contains_key(&key) {
            self.sub_registries.insert(key.clone(), ChangingRegistry::new());
        }
        self.sub_registries.get_mut(&key).unwrap()
    }
}

impl StateRegistry for ChangingRegistry {
    fn set(&mut self, key: impl ToString, value: State) {
        let key = key.to_string();
        if !self.states.contains_key(&key) {
            self.states.insert(key.clone(), Changing::new(value.clone()));
        }
        let v = self.states.get_mut(&key).unwrap();
        v.set(value);
    }

    fn get(&self, key: impl ToString) -> &State {
        let key = key.to_string();
        match self.states.get(&key) {
            None => &State::None,
            Some(state) => state.current(),
        }
    }
}

pub trait StyleTrait {
    fn styles(&mut self) -> &mut UnchangingRegistry;
}