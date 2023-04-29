use crate::prelude::*;
use antlr_rust::tree::ParseTreeListener;
use rusty_yarn_spinner_core::prelude::*;
mod emit;
use crate::parser::generated::yarnspinnerparser::YarnSpinnerParserContextType;
use crate::prelude::generated::yarnspinnerparserlistener::YarnSpinnerParserListener;
pub(crate) use emit::*;

#[derive(Debug)]
pub(crate) struct CompilerListener {
    /// The current node to which instructions are being added.
    pub(crate) current_node: Option<Node>,
    /// The current debug information that describes [`current_node`].
    pub(crate) current_debug_info: DebugInfo,
}

impl<'input> ParseTreeListener<'input, YarnSpinnerParserContextType> for CompilerListener {}

impl<'input> YarnSpinnerParserListener<'input> for CompilerListener {}
