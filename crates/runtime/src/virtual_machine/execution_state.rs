//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs, which we split into multiple files

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[allow(dead_code)]
pub(crate) enum ExecutionState {
    /// The VirtualMachine is not running a node.
    #[default]
    Stopped,

    /// The VirtualMachine is waiting on option selection. Call
    /// [`VirtualMachine::set_selected_option`] before calling
    /// [`VirtualMachine::continue_`].
    WaitingOnOptionSelection,

    /// The VirtualMachine has finished delivering content to the
    /// client game, and is waiting for [`VirtualMachine::continue_`]
    /// to be called.
    WaitingForContinue,

    /// The VirtualMachine is delivering a line, options, or a
    /// command to the client game.
    DeliveringContent,

    /// The VirtualMachine is in the middle of executing code.
    Running,
}
