//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/DebugInfo.cs>

use crate::prelude::*;
use std::collections::HashMap;

/// Contains debug information for a node in a Yarn file.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Default))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct DebugInfo {
    /// The file that this DebugInfo was produced from.
    pub file_name: String,

    /// The node that this DebugInfo was produced from.
    pub node_name: String,

    /// The mapping of instruction numbers to line and character
    /// information in the file indicated by `file_name`.
    pub line_positions: HashMap<usize, Option<Position>>,
}

impl DebugInfo {
    /// Gets a [`LineInfo`] object that describes the specified instruction
    /// at the index `instruction_number`.
    ///
    /// # Arguments
    ///
    /// * `instruction_number` - The index of the instruction to retrieve
    ///  information for.
    ///
    /// # Returns
    ///
    /// A [`LineInfo`] object that describes the position of the instruction.
    ///
    /// # Panics
    ///
    /// Panics if `instruction_number` is greater than the
    /// number of instructions present in the node.
    ///
    /// # See also
    ///
    /// * [`DebugInfo::try_get_line_info`] for the fallible version.
    pub fn get_line_info(&self, instruction_number: usize) -> LineInfo {
        self.try_get_line_info(instruction_number)
            .expect("instruction_number out of range")
    }

    /// Fallible version of [`DebugInfo::get_line_info`].
    pub fn try_get_line_info(&self, instruction_number: usize) -> Option<LineInfo> {
        self.line_positions
            .get(&instruction_number)
            .map(|position| LineInfo {
                file_name: self.file_name.clone(),
                node_name: self.node_name.clone(),
                position: *position,
            })
    }
}

/// Contains positional information about an instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct LineInfo {
    /// The file name of the source that this instruction was produced from.
    pub file_name: String,

    /// The node name of the source that this instruction was produced from.
    pub node_name: String,

    /// The zero-indexed position in `file_name` that contains the
    /// statement or expression that this line was produced from.
    pub position: Option<Position>,
}
