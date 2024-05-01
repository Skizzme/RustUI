use std::cmp::{max, min};
use std::time::{Duration, Instant};
use crate::screen::Screen;

pub enum AnimationType {
    Linear,
    CubicIn,
    CubicOut,
}

impl AnimationType {
    fn get_value(&self, state: f64) -> f64 {
        match self {
            AnimationType::Linear => {
                state
            }
            AnimationType::CubicIn => {
                state * state * state
            }
            AnimationType::CubicOut => {
                1.0 - (state * state * state)
            }
        }
    }
}

pub struct Animation {
    target: f64,
    starting: f64,
    value: f64,
    state: f64,
    start_state: f64,
}

impl Animation {
    pub fn new() -> Self {
        Animation {
            target: 0.0,
            starting: 0.0,
            value: 0.0,
            state: 0.0,
            start_state: 0.0,
        }
    }

    pub fn animate(&mut self, target: f64, mut speed: f64, animation_type: AnimationType, screen: &Screen) -> f64 {
        if self.target != target {
            self.target = target;
            self.starting = self.value;
            self.start_state = self.state;
        }

        println!("{}, {}, {}, {}", (self.value - self.starting).abs(), (self.target - self.starting).abs(), (self.target - self.starting), self.starting);
        self.value = animation_type.get_value((self.value - self.starting).abs() / (self.target - self.starting).abs()) * (self.target - self.starting) + self.starting;

        if self.target < self.value {
            self.state -= speed*screen.frame_delta;
            if self.value < self.target {self.value = self.target;}
        } else if self.target > self.value {
            self.state += speed*screen.frame_delta;
            if self.value > self.target {self.value = self.target;}
        }

        self.value
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}