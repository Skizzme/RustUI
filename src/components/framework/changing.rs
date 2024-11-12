/// A struct for easy tracking of whether a value has changed since the last check / update.
///
/// Eliminates the need for something like `value`, `last_value`
pub struct Changing<T> {
    current: T,
    last: T,
}

impl<T: Clone + PartialEq> Changing<T> {
    pub fn new(value: T) -> Self {
        Changing {
            current: value.clone(),
            last: value,
        }
    }

    pub fn set(&mut self, value: T) {
        std::mem::swap(&mut self.current, &mut self.last);
        self.current = value;
    }

    pub fn changed(&self) -> bool {
        !self.current.eq(&self.last)
    }

    pub fn update(&mut self) {
        self.current = self.last.clone();
    }

    pub fn current(&mut self) -> &mut T {
        &mut self.current
    }
    pub fn last(&self) -> &T {
        &self.last
    }
}