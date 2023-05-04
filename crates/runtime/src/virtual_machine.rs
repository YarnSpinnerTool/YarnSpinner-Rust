//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs
//!
//! ## Implementation Notes
//! The `Operand` extensions and the `Operator` enum were moved into upstream crates to make them not depend on the runtime.

use yarn_slinger_core::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct VirtualMachine {
    pub(crate) program: Program,
}

impl VirtualMachine {
    pub(crate) fn reset_state(&mut self) {
        todo!()
    }
}
