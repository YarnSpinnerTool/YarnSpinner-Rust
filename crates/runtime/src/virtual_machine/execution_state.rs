//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs, which we split into multiple files

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum ExecutionState {
    /// The VirtualMachine is not running a node.
    Stopped,

    /// The VirtualMachine is waiting on option selection. Call
    /// [`VirtualMachine::set_selected_option`] before calling
    /// [`VirtualMachine::continue`].
    WaitingOnOptionSelection,

    /// The VirtualMachine has finished delivering content to the
    /// client game, and is waiting for [`VirtualMachine::continue`]
    /// to be called.
    WaitingForContinue,

    /// The VirtualMachine is delivering a line, options, or a
    /// commmand to the client game.
    DeliveringContent,

    /// The VirtualMachine is in the middle of executing code.
    Running,
}
