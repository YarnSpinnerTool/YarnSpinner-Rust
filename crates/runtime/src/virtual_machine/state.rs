//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs, which we split into multiple files

use crate::prelude::*;
use yarn_slinger_compiler::collections::Stack;
use yarn_slinger_core::prelude::*;

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub(crate) struct State {
    /// The name of the node that we're currently in.
    pub(crate) current_node_name: Option<NodeName>,

    /// The instruction number in the current node.
    pub(crate) program_counter: usize,

    /// The current list of options that will be delivered
    /// when the next RunOption instruction is encountered.
    pub(crate) current_options: Vec<DialogueOption>,

    /// The value stack.
    pub(crate) stack: Stack<InternalValue>,
}

#[allow(dead_code)]
impl State {
    pub(crate) fn push(&mut self, value: impl Into<InternalValue>) {
        self.stack.push(value.into())
    }

    pub(crate) fn pop(&mut self) -> Option<InternalValue> {
        self.stack.pop()
    }

    pub(crate) fn peek(&self) -> Option<&InternalValue> {
        self.stack.peek()
    }
}
