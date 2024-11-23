use std::collections::HashMap;

use gl::{BLEND, DEPTH, Disable, Enable, TEXTURE_2D};

use crate::components::context::context;
use crate::gl_binds::gl11::{Scalef, Translatef};
use crate::gl_binds::gl11::types::GLenum;

unsafe fn enable_disable(state: GLenum, value: bool) {
    match value {
        true => Enable(state),
        false => Disable(state)
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Depth(bool),
    Texture2D(bool),
    Blend(bool),
    Translate(f32, f32),
    Scale(f32, f32),
}

impl State {
    pub unsafe fn apply(&self) {
        match *self {
            State::Depth(v) => enable_disable(DEPTH, v),
            State::Texture2D(v) => enable_disable(TEXTURE_2D, v),
            State::Blend(v) => enable_disable(BLEND, v),
            State::Translate(x, y) => {
                context().window().mouse.pos += (-x, -y);
                Translatef(x, y, 0.0);
            },
            State::Scale(x, y) => {
                Scalef(x, y, 1.0);
            }
        }
    }

    pub unsafe fn unapply(&self) {
        match *self {
            State::Depth(v) => enable_disable(DEPTH, !v),
            State::Texture2D(v) => enable_disable(TEXTURE_2D, !v),
            State::Blend(v) => enable_disable(BLEND, !v),
            State::Translate(x, y) => {
                context().window().mouse.pos += (x, y);
                Translatef(-x, -y, 0.0);
            }
            State::Scale(x, y) => {
                Scalef(1.0/x, 1.0/y, 1.0);
            }
        }
    }

    pub fn same(&self, other: &State) -> bool {
        match self {
            State::Depth(v1) => match other {
                State::Depth(v2) => v1 == v2,
                _ => false
            }
            State::Texture2D(v1) => match other {
                State::Texture2D(v2) => v1 == v2,
                _ => false
            }
            State::Blend(v1) => match other {
                State::Blend(v2) => v1 == v2,
                _ => false
            },
            State::Translate(x1, y1) => match other {
                State::Translate(x2, y2) => x1 == x2 && y1 == y2,
                _ => false,
            },
            State::Scale(x1, y1) => match other {
                State::Scale(x2, y2) => x1 == x2 && y1 == y2,
                _ => false,
            },
        }
    }

    pub fn id(&self) -> u8 {
        match self {
            State::Depth(_) => 0,
            State::Texture2D(_) => 1,
            State::Blend(_) => 2,
            State::Translate(_, _) => 3,
            State::Scale(_, _) => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlState {
    applied: bool,
    level: u8,
    state: State,
}

impl GlState {
    pub fn new(level: u8, state: State) -> Self {
        GlState {
            level,
            state,
            applied: false,
        }
    }

    pub unsafe fn apply(&mut self) {
        self.applied = true;
        self.state.apply();
    }

    pub unsafe fn unapply(&mut self) {
        self.applied = false;
        self.state.unapply();
    }

    pub fn applied(&self) -> bool {
        self.applied
    }
}

#[derive(Debug)]
pub struct Stack {
    stack: Vec<GlState>,
    markers: Vec<usize>,
    current: HashMap<u8, GlState>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: vec![],
            markers: vec![],
            current: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.current.clear();
        self.markers.clear();
    }

    pub fn begin(&mut self) {
        self.markers.push(self.stack.len())
    }

    pub unsafe fn push(&mut self, state: State) {
        self.push_l(state, 0);
    }

    pub unsafe fn push_l(&mut self, state: State, level: u8) {
        let mut state = GlState::new(level, state);
        let id = state.state.id();
        if !self.current.contains_key(&id) {
            self.current.insert(id, state.clone());
            state.apply();
        } else {
            // let current = self.current.get(&id).unwrap();
            // TODO state checks properly
            // if !current.state.same(&state.state) && current.level <= state.level {
                state.apply();
            // }
        }

        self.stack.push(state)
    }

    /// Pop all states since the most recent marker
    pub unsafe fn end(&mut self) {
        match self.markers.last() {
            Some(index) => {
                // println!("end mark {} {}", index, self.stack.len());
                for _ in 0..(self.stack.len()-index) {
                    self.pop();
                }
                // for i in self.stack.len()..=*index {
                //     let mut state = self.stack.remove(i);
                //     state.unapply();
                //     self.current.remove(&state.state.id());
                // }
            }
            None => {
                println!("markers is empty")
            }
        }
    }

    pub unsafe fn pop(&mut self) -> Option<GlState> {
        match self.stack.pop() {
            None => {
                println!("popped on empty stack");
                None
            }
            Some(mut state) => {
                if state.applied() {
                    state.unapply();
                    // TODO this is probably not proving the proper functionality
                    self.current.remove(&state.state.id());
                }
                Some(state)
            }
        }
    }
}