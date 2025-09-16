use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

pub struct DropStack<T> {
    stack: VecDeque<T>,
    max_size: usize,
}

impl<T> DropStack<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            stack: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push_top(&mut self, v: T) {
        if self.stack.len() == self.max_size {
            self.stack.pop_front();
        }
        self.stack.push_back(v);
    }
}

impl<T> Deref for DropStack<T> {
    type Target = VecDeque<T>;
    fn deref(&self) -> &Self::Target {
        &self.stack
    }
}
impl<T> DerefMut for DropStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stack
    }
}
