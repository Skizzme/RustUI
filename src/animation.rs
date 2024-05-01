use std::cmp::{max, min};
use std::f64::consts::PI;
use std::time::{Duration, Instant};
use crate::screen::Screen;

pub enum AnimationType {
    Linear,
    Log,
    CubicIn,
    CubicOut,
    QuarticIn,
    QuarticOut,
    Progressive(f64),
    Sin,
}

impl AnimationType {
    fn get_value(&self, state: f64) -> f64 {
        match self {
            AnimationType::Linear => {
                state
            }
            AnimationType::Log => {
                ((state + 0.01).log10()+2.0)/2.0
            }
            AnimationType::CubicIn => {
                (state - 1.0).powf(3.0) + 1.0
            }
            AnimationType::CubicOut => {
                state.powf(3.0)
            }
            AnimationType::QuarticOut => {
                state.powf(4.0)
            }
            AnimationType::QuarticIn => {
                (state - 1.0).powf(4.0)
            }
            AnimationType::Sin => {
                (state * PI/2.0).sin()
            }
            AnimationType::Progressive(speed) => {
                2.0/(1.0+20f64.powf(-(20.0/speed)*state))-1.0
            }
        }.clamp(0.0, 1.0)
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
            self.state = 0f64;
        }

        self.state += speed*screen.frame_delta;
        if self.state > 1.0 {self.state = 1.0}

        self.value = animation_type.get_value(self.state)*(self.target-self.starting)+self.starting;

        self.value
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}