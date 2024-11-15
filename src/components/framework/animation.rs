use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::rc::Rc;

use rand::random;

use crate::components::context::context;

/// Different animation types will give different animation curves, and provide a cleaner visual than `linear`
pub enum AnimationType {
    Linear,
    Log,
    CubicIn,
    CubicOut,
    QuarticIn,
    QuarticOut,
    Progressive(f32),
    Sin,
}

impl AnimationType {
    fn get_value(&self, state: f32) -> f32 {
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
                (state * (PI/2.0) as f32).sin()
            }
            AnimationType::Progressive(speed) => {
                2.0/(1.0+20f32.powf(-(20.0/speed)*state))-1.0
            }
        }.clamp(0.0, 1.0)
    }
}

/// An easy-to-use object to animate objects over time
#[derive(Debug, Clone, Copy)]
pub struct Animation {
    id: u32,
    target: f32,
    starting: f32,
    value: f32,
    last_value: f32,
    state: f32,
}

impl Animation {
    pub fn new() -> Self {
        Animation {
            id: random::<u32>(),
            target: 0.0,
            starting: 0.0,
            value: 0.0,
            last_value: 0.0,
            state: 0.0,
        }
    }

    pub unsafe fn animate_to(&mut self, target: f32, speed: f32, animation_type: AnimationType) -> f32 {
        self.set_target(target);

        self.animate(speed, animation_type)
    }

    pub unsafe fn animate(&mut self, speed: f32, animation: AnimationType) -> f32 {
        self.state += speed * context().framework().pre_delta();
        self.state = self.state.clamp(0.0, 1.0);

        self.value = animation.get_value(self.state)*(self.target-self.starting)+self.starting;

        self.value
    }

    pub unsafe fn set_target(&mut self, target: f32) {
        if self.target != target {
            self.target = target;
            self.starting = self.value;
            self.state = 0f32;
        }
    }

    pub(super) fn has_changed(&self) -> bool {
        // println!("changed? {} {} {}", self.last_value, self.value, self.last_value != self.value);
        self.last_value != self.value
    }

    pub(super) fn post(&mut self) {
        self.last_value = self.value;
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn target(&self) -> f32 {
        self.target
    }
    pub fn starting(&self) -> f32 {
        self.starting
    }
    pub fn value(&self) -> f32 {
        self.value
    }
    pub fn state(&self) -> f32 {
        self.state
    }
}

pub struct AnimationRegistry {
    animations: HashMap<u32, AnimationRef>,
}

impl AnimationRegistry {
    pub fn new() -> Self {
        AnimationRegistry {
            animations: HashMap::new(),
        }
    }

    pub fn new_anim(&mut self) -> AnimationRef {
        let rc = Rc::new(RefCell::new(Animation::new()));
        self.animations.insert(rc.borrow().id(), rc.clone());
        rc
    }

    pub fn register(&mut self, animation: AnimationRef) {
        let id = animation.borrow().id;
        self.animations.insert(id, animation);
    }

    // pub fn register_ref(&mut self, animation: AnimationRef) {
    //     self.animations.insert(animation.borrow().id(), animation);
    // }

    pub fn unregister(&mut self, animation: &AnimationRef) {
        self.animations.remove(&animation.borrow().id());
    }

    pub fn get(&mut self, id: u32) -> Option<AnimationRef> {
        self.animations.get(&id).and_then(|rc| Some(rc.clone()))
    }

    pub fn all(&self) -> Vec<AnimationRef> {
        self.animations.values().map(|v| v.clone()).collect()
    }

    pub fn has_changed(&self) -> bool {
        let mut result = false; // so that all animations can be checked, meaning all queries are current
        for anim in self.animations.values() {
            if anim.borrow().has_changed() {
                result = true;
            }
        }
        result
    }

    pub fn post(&mut self) {
        for anim in self.animations.values() {
            anim.borrow_mut().post();
        }
    }
}

impl AnimationRegTrait for AnimationRegistry {
    fn animations(&mut self) -> &mut AnimationRegistry {
        self
    }
}

pub type AnimationRef = Rc<RefCell<Animation>>;

/// Keeps all relevant animations for something in one place,
/// allowing the framework to determine whether to render or not
pub trait AnimationRegTrait {
    fn animations(&mut self) -> &mut AnimationRegistry;
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