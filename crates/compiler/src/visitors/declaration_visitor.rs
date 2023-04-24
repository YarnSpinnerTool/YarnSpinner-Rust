//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/DeclarationVisitor.cs>

use crate::parser::generated::yarnspinnerparser::{Declare_statementContext, HashtagContext};
use crate::prelude::generated::yarnspinnerparser::{
    NodeContext, NodeContextAttrs, YarnSpinnerParserContextType,
};
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::{Declaration, Diagnostic};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token::Token;
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

    /// The name of the file we're currently in.
    source_file_name: String,

    /// Gets the collection of types known to this
    ///  [`DeclarationVisitor`].
    types: Vec<Type>,

    keywords_to_builtin_types: HashMap<&'static str, BuiltinType>,
    /// A regular expression used to detect illegal characters in node titles.
    regex: Regex,

    _dummy: Option<BuiltinType>,
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
    type Return = Option<BuiltinType>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'input, T: TokenSource<'input>> YarnSpinnerParserVisitorCompat<'input>
    for DeclarationVisitor<'input, T>
{
    fn visit_hashtag(&mut self, ctx: &HashtagContext<'input>) -> Self::Return {
        let hashtag_text = ctx.text.as_ref().unwrap();
        self.file_tags.push(hashtag_text.get_text().to_owned());
        None
    }

    fn visit_node(&mut self, ctx: &NodeContext<'input>) -> Self::Return {
        for header in ctx.header_all() {
            let header_key = header.header_key.as_ref().unwrap();
            if header_key.get_text() != "title" {
                continue;
            }

            let header_value = header.header_value.as_ref().unwrap();
            let current_node_name = header_value.get_text();
            self.current_node_name = Some(current_node_name.to_owned());
            if self.regex.is_match(current_node_name) {
                let message =
                    format!("The node '{current_node_name}' contains illegal characters.");
                self.diagnostics.push(
                    Diagnostic::from_message(message)
                        .with_file_name(self.source_file_name.clone())
                        .read_parser_rule_context(&*header),
                );
            }
        }
        if let Some(body) = ctx.body() {
            self.visit(&*body);
        }
        None
    }
}
