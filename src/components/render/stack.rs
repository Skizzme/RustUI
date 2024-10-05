use std::collections::HashMap;
use gl::{BLEND, DEPTH, Disable, Enable, TEXTURE_2D};
use crate::gl_binds::gl11::types::GLenum;

unsafe fn enable_disable(state: GLenum, value: bool) {
    if value {
        println!("enable {}", state);
        Enable(state);
    } else {
        println!("disable {}", state);
        Disable(state);
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Depth(bool),
    Texture2D(bool),
    Blend(bool),
}

impl State {
    pub unsafe fn apply(&self) {
        match self {
            State::Depth(v) => enable_disable(DEPTH, *v),
            State::Texture2D(v) => enable_disable(TEXTURE_2D, *v),
            State::Blend(v) => enable_disable(BLEND, *v),
        }
        // println!("applied {:?}", self);
    }

    pub unsafe fn unapply(&self) {
        match self {
            State::Depth(v) => enable_disable(DEPTH, !*v),
            State::Texture2D(v) => enable_disable(TEXTURE_2D, !*v),
            State::Blend(v) => enable_disable(BLEND, !*v),
        }
        // println!("unapplied {:?}", self);
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
            }
        }
    }

    pub fn id(&self) -> u8 {
        match self {
            State::Depth(_) => 0,
            State::Texture2D(_) => 1,
            State::Blend(_) => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct GlState {
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

    pub unsafe fn push(&mut self, mut state: State) {
        self.push_l(GlState::new(0, state));
    }

    pub unsafe fn push_l(&mut self, mut state: GlState) {
        let id = state.state.id();
        if !self.current.contains_key(&id) {
            self.current.insert(id, state.clone());
            println!("apply {:?}", state.state);
            state.apply();
        } else {
            let current = self.current.get(&id).unwrap();
            println!("{:?} {:?}", current.state, state.state);
            if !current.state.same(&state.state) && current.level <= state.level {
                state.apply();
            }
        }

        self.stack.push(state)
    }

    /// Pop all states since the most recent marker
    pub unsafe fn end(&mut self) {
        match self.markers.last() {
            Some(index) => {
                for i in self.stack.len()..=*index {
                    let mut state = self.stack.remove(i);
                    state.unapply();
                    self.current.remove(&state.state.id());
                }
            }
            None => {
                println!("markers is empty")
            }
        }
    }

    pub unsafe fn pop(&mut self) {
        match self.stack.pop() {
            None => {
                println!("popped on empty stack")
            }
            Some(mut state) => {
                if state.applied() {
                    state.unapply();
                    // TODO this is probably not proving the proper functionality
                    self.current.remove(&state.state.id());
                }
            }
        }
    }
}