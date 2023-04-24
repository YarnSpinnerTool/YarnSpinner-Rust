use std::{collections::VecDeque, fmt::Debug};

/// Represents a FIFO (First-In, First-Out) collection.
///
/// Models the behaviour of <https://learn.microsoft.com/en-us/dotnet/api/system.collections.generic.queue-1>
#[derive(Debug, Clone)]
pub(crate) struct Queue<T: Debug + Clone>(pub(crate) VecDeque<T>);

impl<T: Debug + Clone> Queue<T> {
    pub(crate) fn enqueue(&mut self, value: T) {
        self.0.push_back(value)
    }

    pub(crate) fn dequeue(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

/// Represents a FILO (First-In, Last-Out) collection.
///
/// Models the behaviour of <https://learn.microsoft.com/en-us/dotnet/api/system.collections.generic.stack-1>
#[derive(Debug, Clone)]
pub(crate) struct Stack<T: Debug + Clone>(pub(crate) VecDeque<T>);

impl<T: Debug + Clone> Stack<T> {
    pub(crate) fn push(&mut self, value: T) {
        self.0.push_back(value)
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

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
