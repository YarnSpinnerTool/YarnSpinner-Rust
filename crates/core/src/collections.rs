//! Thin newtypes around existing collections to better express their intent in regards to the corresponding dotnet types.

use std::{collections::VecDeque, fmt::Debug};

/// Represents a FIFO (First-In, First-Out) collection.
///
/// Models the behaviour of <https://learn.microsoft.com/en-us/dotnet/api/system.collections.generic.queue-1>
#[derive(Debug, Clone)]
pub struct Queue<T: Debug + Clone>(pub VecDeque<T>);

impl<T: Debug + Clone> Queue<T> {
    pub fn enqueue(&mut self, value: T) {
        self.0.push_back(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}

/// Represents a FILO (First-In, Last-Out) collection.
///
/// Models the behaviour of <https://learn.microsoft.com/en-us/dotnet/api/system.collections.generic.stack-1>
#[derive(Debug, Clone)]
pub struct Stack<T: Debug + Clone>(pub VecDeque<T>);

impl<T: Debug + Clone> Stack<T> {
    pub fn push(&mut self, value: T) {
        self.0.push_back(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_back()
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.back()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}

// Somehow auto-derive does not work with those types so a manual impl is needed?

impl<T: Debug + Clone> Default for Queue<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Debug + Clone> Default for Stack<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
