//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/DeclarationVisitor.cs>

use crate::compiler;
use crate::parser::generated::yarnspinnerparser::{Declare_statementContext, HashtagContext};
use crate::prelude::generated::yarnspinnerparser::{
    Declare_statementContextAttrs, NodeContext, NodeContextAttrs, YarnSpinnerParserContextType,
};
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::{Declaration, Diagnostic};
use crate::visitors::constant_value_visitor::ConstantValueVisitor;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token::Token;
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use antlr_rust::TokenSource;
use regex::Regex;
use rusty_yarn_spinner_core::types::{BooleanType, BuiltinType, NumberType, StringType, Type};
use std::borrow::Cow;
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

    _dummy: (),
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
where
    <T::TF as TokenFactory<'input>>::Tok: Token<Data = Cow<'input, str>>,
{
    type Node = YarnSpinnerParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'input, T: TokenSource<'input>> YarnSpinnerParserVisitorCompat<'input>
    for DeclarationVisitor<'input, T>
where
    <T::TF as TokenFactory<'input>>::Tok: Token<Data = Cow<'input, str>>,
{
    fn visit_hashtag(&mut self, ctx: &HashtagContext<'input>) -> Self::Return {
        let hashtag_text = ctx.text.as_ref().unwrap();
        self.file_tags.push(hashtag_text.get_text().to_owned());
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
    }

    fn visit_declare_statement(&mut self, ctx: &Declare_statementContext<'input>) -> Self::Return {
        compiler::get_document_comments(&self.tokens, ctx);

        // Get the name of the variable we're declaring
        let variable_context = ctx.variable().unwrap();
        let variable_name = variable_context.get_text();

        // Does this variable name already exist in our declarations?
        let existing_explicit_declaration = self
            .declarations()
            .into_iter()
            .find(|d| !d.is_implicit && d.name == variable_name);
        if let Some(existing_explicit_declaration) = existing_explicit_declaration {
            // Then this is an error, because you can't have two explicit declarations for the same variable.
            let msg = format!(
                "{} has already been declared in {}, line {}",
                existing_explicit_declaration.name,
                existing_explicit_declaration.source_file_name,
                existing_explicit_declaration.source_file_line.unwrap(),
            );
            self.diagnostics.push(
                Diagnostic::from_message(msg)
                    .with_file_name(&self.source_file_name)
                    .read_parser_rule_context(&*ctx),
            );
            return;
        }

        // Figure out the value and its type
        let mut constant_value_visitor =
            ConstantValueVisitor::new(self.source_file_name.clone(), self.diagnostics.clone());
        let value = constant_value_visitor.visit(&*ctx.value().unwrap());
        self.diagnostics
            .extend_from_slice(&constant_value_visitor.diagnostics);

        // Did the source code name an explicit type?
        if let Some(declaration_type) = ctx.declaration_type.as_ref() {
            let Some(explicitType)  = self.keywords_to_builtin_types.get(declaration_type.get_text()) else {
                // The type name provided didn't map to a built-in
                // type. Look for the type in our Types collection.
                if let Some(explicitType) = self.types.iter().find(|t| t.name == declaration_type.get_text()) {
                    explicitType.into()
                } else {
                    // We didn't find a type by this name.
                    let msg = format!("Unknown type {}", declaration_type.get_text());
                    self.diagnostics.push(
                        Diagnostic::from_message(msg)
                            .with_file_name(&self.source_file_name)
                            .read_parser_rule_context(&*ctx),
                    );
                    return;
                }
            };

            // Check that the type we've found is compatible with the
            // type of the value that was provided - if it doesn't,
            // that's a type error
             if (TypeUtil.IsSubType(explicitType, value.Type) == false)
               {
                   string v = $"Type {context.type.Text} does not match value {context.value().GetText()} ({value.Type.Name})";
                   this.diagnostics.Add(new Diagnostic(this.sourceFileName, context, v));
                   return BuiltinTypes.Undefined;
               }
        }

        /*
           // We're done creating the declaration!

           // The start line of the body is the line after the delimiter
           // Hohenheim: Pretty sure we can remove this
           _ = this.currentNodeContext.BODY_START().Symbol.Line;

           string description = Compiler.GetDocumentComments(tokens, context);
           var declaration = new Declaration
           {
               Name = variableName,
               Type = value.Type,
               DefaultValue = value.InternalValue,
               Description = description,
               SourceFileName = this.sourceFileName,
               SourceNodeName = this.currentNodeName,
               Range = new Range
               {
                   Start =
                   {
                       Line = variableContext.Start.Line - 1,
                       Character = variableContext.Start.Column,
                   },
                   End =
                   {
                       Line = variableContext.Stop.Line - 1,
                       Character = variableContext.Stop.Column + variableContext.GetText().Length,
                   },
               },
               IsImplicit = false,
           };

           this.NewDeclarations.Add(declaration);

           return value.Type;
        */
    }
}
