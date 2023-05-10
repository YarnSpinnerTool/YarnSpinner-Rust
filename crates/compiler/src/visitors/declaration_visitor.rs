//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/DeclarationVisitor.cs>

use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::*;
use crate::visitors::constant_value_visitor::ConstantValueVisitor;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use regex::Regex;
use yarn_slinger_core::types::*;

/// A visitor that extracts variable declarations from a parse tree.
/// After visiting an entire parse tree for a file, the
/// [`NewDeclarations`] property will contain all explicit
/// variable declarations that were found.
pub(crate) struct DeclarationVisitor<'input> {
    /// Gets the collection of new variable declarations that were
    /// found as a result of using this
    /// [`DeclarationVisitor`] to visit a
    /// [`ParserRuleContext`].
    pub(crate) new_declarations: Vec<Declaration>,

    /// Gets the collection of file-level hashtags that were found as a
    /// result of using this  [`DeclarationVisitor`] to visit a [`ParserRuleContext`].
    pub(crate) file_tags: Vec<String>,

    pub(crate) diagnostics: Vec<Diagnostic>,

    /// The CommonTokenStream derived from the file we're parsing. This
    /// is used to find documentation comments for declarations.
    file: FileParseResult<'input>,

    /// The collection of variable declarations we know about before starting our work
    existing_declarations: Vec<Declaration>,

    /// The name of the node that we're currently visiting.
    current_node_name: Option<String>,

    /// A regular expression used to detect illegal characters in node titles.
    regex: Regex,

    _dummy: (),
}

impl<'input> DeclarationVisitor<'input> {
    pub(crate) fn new(
        existing_declarations: Vec<Declaration>,
        file: FileParseResult<'input>,
    ) -> Self {
        Self {
            file,
            existing_declarations,
            new_declarations: Default::default(),
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

impl<'input> ParseTreeVisitorCompat<'input> for DeclarationVisitor<'input> {
    type Node = YarnSpinnerParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'input> YarnSpinnerParserVisitorCompat<'input> for DeclarationVisitor<'input> {
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
                        .with_file_name(self.file.name.clone())
                        .with_parser_context(header.as_ref(), self.file.tokens()),
                );
            }
        }
        if let Some(body) = ctx.body() {
            self.visit(body.as_ref());
        }
    }

    fn visit_file_hashtag(&mut self, ctx: &File_hashtagContext<'input>) -> Self::Return {
        let hashtag_text = ctx.text.as_ref().unwrap();
        self.file_tags.push(hashtag_text.get_text().to_owned());
    }

    fn visit_declare_statement(&mut self, ctx: &Declare_statementContext<'input>) -> Self::Return {
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
            let line = existing_explicit_declaration
                .source_file_line()
                .map(|l| format!(", line: {l}"))
                .unwrap_or_default();
            let msg = format!(
                "{} has already been declared in {}{line}",
                existing_explicit_declaration.name, existing_explicit_declaration.source_file_name,
            );
            self.diagnostics.push(
                Diagnostic::from_message(msg)
                    .with_file_name(&self.file.name)
                    .with_parser_context(ctx, self.file.tokens()),
            );
            return;
        }

        // Figure out the value and its type
        let mut constant_value_visitor =
            ConstantValueVisitor::new(self.diagnostics.clone(), self.file.clone());
        let value_context = ctx.value().unwrap();
        let value = constant_value_visitor.visit(value_context.as_ref());
        self.diagnostics
            .extend_from_slice(&constant_value_visitor.diagnostics);

        // Did the source code name an explicit type?
        if let Some(declaration_type) = ctx.declaration_type.as_ref() {
            let explicit_type = match keyword_to_type(declaration_type.get_text()) {
                Some(builtin_type) => builtin_type,

                // The type name provided didn't map to a built-in
                // type. Look for the type in our type collection.
                None => match Type::EXPLICITLY_CONSTRUCTABLE
                    .iter()
                    .find(|t| t.to_string() == declaration_type.get_text())
                {
                    Some(explicit_type) => explicit_type.clone(),
                    None => {
                        // We didn't find a type by this name.
                        let msg = format!("Unknown type {}", declaration_type.get_text());
                        self.diagnostics.push(
                            Diagnostic::from_message(msg)
                                .with_file_name(&self.file.name)
                                .with_parser_context(ctx, self.file.tokens()),
                        );
                        return;
                    }
                },
            };

            // Check that the type we've found is compatible with the
            // type of the value that was provided - if it doesn't,
            // that's a type error
            if let Some(value) = value.as_ref() {
                if !value.r#type.is_sub_type_of(&explicit_type) {
                    let msg = format!(
                        "Type {} does not match value {} ({})",
                        declaration_type.get_text(),
                        value_context.get_text(),
                        value.r#type.format()
                    );
                    self.diagnostics.push(
                        Diagnostic::from_message(msg)
                            .with_file_name(&self.file.name)
                            .with_parser_context(ctx, self.file.tokens()),
                    );
                    return;
                }
            }
        }
        // We're done creating the declaration!
        let description = get_document_comments(self.file.tokens(), ctx);
        let description_as_option = (!description.is_empty()).then_some(description);
        if let Some(value) = value.as_ref() {
            let declaration = Declaration::new(variable_name, value.r#type.clone())
                .with_default_value(value.raw_value.clone())
                .with_description_optional(description_as_option)
                .with_source_file_name(self.file.name.clone())
                .with_source_node_name_optional(self.current_node_name.clone())
                .with_range(variable_context.range());
            self.new_declarations.push(declaration);
        }
    }
}

fn keyword_to_type(keyword: &str) -> Option<Type> {
    match keyword {
        "string" => Some(Type::String),
        "number" => Some(Type::Number),
        "bool" => Some(Type::Boolean),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_variable_declarations() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
<<declare $foo to 1>>
<<declare $bar = \"2\">>
<<declare $baz to true>>
<<declare $quux = \"hello there\" as string>>
==="
            .to_string(),
        };
        let result = Compiler {
            files: vec![file],
            library: Default::default(),
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        }
        .compile()
        .unwrap();

        assert!(result.warnings.is_empty());
        assert_eq!(result.declarations.len(), 4);
        assert_eq!(
            result.declarations[0],
            Declaration::new("$foo", Type::Number)
                .with_default_value(1.0)
                .with_source_file_name("test.yarn")
                .with_source_node_name("test")
                .with_range(
                    Position {
                        line: 2,
                        character: 10,
                    }..Position {
                        line: 2,
                        character: 14,
                    }
                )
        );

        assert_eq!(
            result.declarations[1],
            Declaration::new("$bar", Type::String)
                .with_default_value("2")
                .with_source_file_name("test.yarn")
                .with_source_node_name("test")
                .with_range(
                    Position {
                        line: 3,
                        character: 10,
                    }..Position {
                        line: 3,
                        character: 14,
                    }
                )
        );

        assert_eq!(
            result.declarations[2],
            Declaration::new("$baz", Type::Boolean)
                .with_default_value(true)
                .with_source_file_name("test.yarn")
                .with_source_node_name("test")
                .with_range(
                    Position {
                        line: 4,
                        character: 10,
                    }..Position {
                        line: 4,
                        character: 14,
                    }
                )
        );

        assert_eq!(
            result.declarations[3],
            Declaration::new("$quux", Type::String)
                .with_default_value("hello there")
                .with_source_file_name("test.yarn")
                .with_source_node_name("test")
                .with_range(
                    Position {
                        line: 5,
                        character: 10,
                    }..Position {
                        line: 5,
                        character: 15,
                    }
                )
        );
    }

    #[test]
    fn catches_type_errors() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
<<declare $foo to 1 as string>>
==="
            .to_string(),
        };
        let result = Compiler {
            files: vec![file.clone()],
            library: Default::default(),
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        }
        .compile();

        let diagnostics = result.unwrap_err().diagnostics;
        assert_eq!(2, diagnostics.len());
        assert_eq!(
            diagnostics[0],
            Diagnostic::from_message("Type string does not match value 1 (Number)".to_string())
                .with_file_name("test.yarn".to_string())
                .with_context(file.source.clone())
                .with_range(
                    Position {
                        line: 2,
                        character: 0,
                    }..Position {
                        line: 2,
                        character: 31,
                    }
                )
        );
        assert_eq!(
            diagnostics[1],
            Diagnostic::from_message("Can't figure out the type of variable $foo given its context. Specify its type with a <<declare>> statement.".to_string())
                .with_file_name("test.yarn".to_string())
                .with_context(file.source)
                .with_range(
                    Position {
                        line: 2,
                        character: 10,
                    }..Position {
                        line: 2,
                        character: 14,
                    }
                )
        );
    }
}
