//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs, which we split into multiple files

use crate::prelude::*;
use std::fmt::Debug;
use yarn_slinger_core::collections::Stack;
use yarn_slinger_core::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct State {
    /// The name of the node that we're currently in.
    pub(crate) current_node_name: Option<String>,

    /// The instruction number in the current node.
    pub(crate) program_counter: usize,

    /// The current list of options that will be delivered
    /// when the next RunOption instruction is encountered.
    pub(crate) current_options: Vec<DialogueOption>,

    /// The value stack.
    pub(crate) stack: Stack<InternalValue>,
}

impl State {
    pub(crate) fn push(&mut self, value: impl Into<InternalValue>) {
        self.stack.push(value.into())
    }

    /// Pops a value from the stack and tries to convert it to the specified type.
    ///
    /// ## Panics
    /// - Panics on an empty stack to mirror C# behavior.
    /// - Panics if the value cannot be converted to the specified type.
    pub(crate) fn pop<T>(&mut self) -> T
    where
        T: TryFrom<InternalValue>,
        <T as TryFrom<InternalValue>>::Error: Debug,
    {
        self.pop_value()
            .try_into()
            .unwrap_or_else(|e| panic!("Failed to convert popped value: {e:?}",))
    }

    /// Pops a value from the stack. Panics on an empty stack to mirror C# behavior.
    pub(crate) fn pop_value(&mut self) -> InternalValue {
        self.stack
            .pop()
            .unwrap_or_else(|| panic!("Tried to pop value, but the stack was empty."))
    }

    /// Peeks the top value of the stack. Panics on an empty stack to mirror C# behavior.
    pub(crate) fn peek<T>(&self) -> T
    where
        T: TryFrom<InternalValue>,
        <T as TryFrom<InternalValue>>::Error: Debug,
    {
        self.peek_value()
            .clone()
            .try_into()
            .unwrap_or_else(|e| panic!("Failed to convert popped value: {e:?}",))
    }

    /// Copies the top value of the stack and tries to convert it to the specified type.
    ///
    /// ## Panics
    /// - Panics on an empty stack to mirror C# behavior.
    /// - Panics if the value cannot be converted to the specified type.
    pub(crate) fn peek_value(&self) -> &InternalValue {
        self.stack
            .peek()
            .unwrap_or_else(|| panic!("Tried to peek value, but the stack was empty."))
    }
}
