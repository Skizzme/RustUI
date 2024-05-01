use std::cmp::{max, min};
use std::time::{Duration, Instant};
use crate::screen::Screen;

pub enum AnimationType {
    Linear,
}

impl AnimationType {
    fn get_value(&self, state: f64) -> f64 {
        match self {
            AnimationType::Linear => {
                state
            }
        }
    }
}

pub struct Animation {
    target: f64,
    starting: f64,
    value: f64,
    state: f64,
    animation_type: AnimationType,
}

impl Animation {
    pub fn new() -> Self {
        Animation {
            target: 0.0,
            starting: 0.0,
            value: 0.0,
            state: 0.0,
            animation_type: AnimationType::Linear,
        }
    }

    pub fn animate(&mut self, target: f64, speed: f64, screen: Screen) -> f64 {
        if self.target != target {
            self.target = target;
            self.starting = self.state;
        }

        if self.target < self.state {
            self.state -= speed*screen.frame_delta;
            if self.state < self.target {self.state = self.target;}
        } else if self.target > self.state {
            self.state += speed*screen.frame_delta;
            if self.state > self.target {self.state = self.target;}
        }

        self.animation_type.
    }
}

pub struct FixedAnimation {
    value: f64,
    pub target: f64,
    duration: Duration,
    init_time: Instant,
    anim_type: AnimationType,
}

impl FixedAnimation {
    pub fn new(value: f64, target: f64, duration: Duration, anim_type: AnimationType) -> Self { // Maybe add a list of targets that are animated to one by one, like keyframes
        FixedAnimation {
            value,
            target,
            duration,
            init_time: Instant::now(),
            anim_type,
        }
    }

    /***
    Returns the state of the animation, a number between 0 and 1
     */
    pub fn get_state(&self) -> f64 {
        let state = self.init_time.elapsed().as_secs_f64() / self.duration.as_secs_f64();
        if state < 0.0 {
            0.0
        } else if state > 1.0 {
            1.0
        } else {
            state
        }
    }

    pub fn get_value(&self) -> f64 {
        self.anim_type.get_value(self)animation.value + (animation.target-animation.value)*animation.get_state()
    }
}