//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/DeclarationVisitor.cs>

use crate::prelude::generated::yarnspinnerparser::{NodeContext, YarnSpinnerParserContextType};
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::{Declaration, Diagnostic};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::TokenSource;
use regex::Regex;
use rusty_yarn_spinner_core::types::{BooleanType, BuiltinType, NumberType, StringType, Type};
use std::collections::HashMap;

/// A visitor that extracts variable declarations from a parse tree.
/// /// After visiting an entire parse tree for a file, the
///  [`NewDeclarations`] property will contain all explicit
/// variable declarations that were found.
pub(crate) struct DeclarationVisitor<'input, T: TokenSource<'input>> {
    /// Gets the collection of new variable declarations that were
    /// found as a result of using this
    ///  [`DeclarationVisitor`] to visit a
    ///  [`ParserRuleContext`].
    pub(crate) new_declarations: Vec<Declaration>,

    /// Gets the collection of file-level hashtags that were found as a
    /// result of using this  [`DeclarationVisitor`] to visit
    /// a  [`ParserRuleContext`].
    pub(crate) file_tags: Vec<String>,

    pub(crate) diagnostics: Vec<Diagnostic>,

    /// The CommonTokenStream derived from the file we're parsing. This
    /// is used to find documentation comments for declarations.
    tokens: CommonTokenStream<'input, T>,

    /// The collection of variable declarations we know about before
    /// starting our work
    existing_declarations: Vec<Declaration>,

    /// The name of the node that we're currently visiting.
    current_node_name: Option<String>,

    /// The context of the node we're currently in.
    current_node_context: Option<NodeContext<'input>>,

    /// The name of the file we're currently in.
    source_file_name: String,

    /// Gets the collection of types known to this
    ///  [`DeclarationVisitor`].
    types: Vec<Type>,

    keywords_to_builtin_types: HashMap<&'static str, BuiltinType>,
    /// A regular expression used to detect illegal characters in node titles.
    regex: Regex,

    _dummy: BuiltinType,
}

impl<'input, T: TokenSource<'input>> DeclarationVisitor<'input, T> {
    pub(crate) fn new(
        source_file_name: impl Into<String>,
        existing_declarations: Vec<Declaration>,
        type_declarations: Vec<Type>,
        tokens: CommonTokenStream<'input, T>,
    ) -> Self {
        Self {
            tokens,
            existing_declarations,
            new_declarations: Default::default(),
            source_file_name: source_file_name.into(),
            types: type_declarations,
            keywords_to_builtin_types: HashMap::from([
                ("string", BuiltinType::String(StringType)),
                ("number", BuiltinType::Number(NumberType)),
                ("bool", BuiltinType::Bool(BooleanType)),
            ]),
            regex: Regex::new(r"[\[<>\]{}|:\s#$]").unwrap(),
            file_tags: Default::default(),
            diagnostics: Default::default(),
            current_node_name: None,
            current_node_context: None,
            _dummy: Default::default(),
        }
    }

    /// The collection of all declarations - both the ones we received
    /// at the start, and the new ones we've derived ourselves.
    pub(crate) fn declarations(&self) -> Vec<Declaration> {
        self.existing_declarations
            .iter()
            .chain(self.new_declarations.iter())
            .cloned()
            .collect()
    }
}

impl<'input, T: TokenSource<'input>> ParseTreeVisitorCompat<'input>
    for DeclarationVisitor<'input, T>
{
    type Node = YarnSpinnerParserContextType;
    type Return = BuiltinType;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'input, T: TokenSource<'input>> YarnSpinnerParserVisitorCompat<'input>
    for DeclarationVisitor<'input, T>
{
}
