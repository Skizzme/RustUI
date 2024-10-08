use std::f64::consts::PI;

/// Different animation types will give different animation curves, and provide a cleaner visual than `linear`
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

/// An easy-to-use object to animate objects over time
#[derive(Debug, Clone, Copy)]
pub struct Animation {
    target: f64,
    starting: f64,
    value: f64,
    state: f64,
}

impl Animation {
    pub fn new() -> Self {
        Animation {
            target: 0.0,
            starting: 0.0,
            value: 0.0,
            state: 0.0,
        }
    }
}



// Updates the animation value for 1 call
//
// Should generally be called every frame
// pub fn animate_target(&mut self, target: f64, speed: f64, animation_type: AnimationType, screen: &Window) -> f64 {
//     self.set_target(target);
//
//     self.state += speed*screen.frame_delta;
//     if self.state > 1.0 {self.state = 1.0}
//
//     self.value = animation_type.get_value(self.state)*(self.target-self.starting)+self.starting;
//
//     self.value
// }
//
// pub fn animate(&mut self, speed: f64, animation_type: AnimationType, screen: &Window) -> f64 {
//     self.animate_target(self.target, speed, animation_type, screen)
// }
//
// pub fn set_target(&mut self, target: f64) -> &mut Self {
//     if self.target != target {
//         self.target = target;
//         self.starting = self.value;
//         self.state = 0f64;
//     }
//     self
// }
//
//
// pub fn value(&self) -> f64 {
//     self.value
// }
// pub fn target(&self) -> f64 {
//     self.target
// }