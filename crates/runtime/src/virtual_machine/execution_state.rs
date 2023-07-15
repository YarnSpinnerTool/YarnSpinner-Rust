//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs>, which we split into multiple files

#[cfg(any(feature = "bevy", feature = "serde"))]
use crate::prelude::*;

/// ## Implementation notes
/// Does not contain `DeliveringContent` since that that state would be used to indicate
/// that a handler is currently running, which we don't have.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Default, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub(crate) enum ExecutionState {
    /// The VirtualMachine is not running a node.
    #[default]
    Stopped,

    /// The VirtualMachine is waiting on option selection. Call
    /// [`VirtualMachine::set_selected_option`] before calling
    /// [`VirtualMachine::next`].
    WaitingOnOptionSelection,

    /// The VirtualMachine has finished delivering content to the
    /// client game, and is waiting for [`VirtualMachine::next`]
    /// to be called.
    WaitingForContinue,

    /// The VirtualMachine is in the middle of executing code.
    Running,
}
