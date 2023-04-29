use crate::prelude::*;
use rusty_yarn_spinner_core::prelude::*;
mod emit;
pub(crate) use emit::*;

#[derive(Debug)]
pub(crate) struct CompilerListener {
    /// The current node to which instructions are being added.
    pub(crate) current_node: Option<Node>,
    /// The current debug information that describes [`current_node`].
    pub(crate) current_debug_info: DebugInfo,
}
