//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/DeclarationVisitor.cs>

use crate::compiler;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::*;
use crate::visitors::constant_value_visitor::ConstantValueVisitor;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use regex::Regex;
use rusty_yarn_spinner_core::types::*;
use std::collections::HashMap;

/// A visitor that extracts variable declarations from a parse tree.
/// /// After visiting an entire parse tree for a file, the
///  [`NewDeclarations`] property will contain all explicit
/// variable declarations that were found.
pub(crate) struct DeclarationVisitor<'a, 'input: 'a> {
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
    tokens: &'a ActualTokenStream<'input>,

    /// The collection of variable declarations we know about before
    /// starting our work
    existing_declarations: Vec<Declaration>,

    /// The name of the node that we're currently visiting.
    current_node_name: Option<String>,

    /// The name of the file we're currently in.
    source_file_name: String,

    /// Gets the collection of types known to this
    ///  [`DeclarationVisitor`].
    types: Vec<BuiltinType>,

    keywords_to_builtin_types: HashMap<&'static str, BuiltinType>,

    /// A regular expression used to detect illegal characters in node titles.
    regex: Regex,

    _dummy: (),
}

impl<'a, 'input: 'a> DeclarationVisitor<'a, 'input> {
    pub(crate) fn new(
        source_file_name: impl Into<String>,
        existing_declarations: Vec<Declaration>,
        type_declarations: Vec<BuiltinType>,
        tokens: &'a ActualTokenStream<'input>,
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
                ("bool", BuiltinType::Boolean(BooleanType)),
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

impl<'a, 'input: 'a> ParseTreeVisitorCompat<'input> for DeclarationVisitor<'a, 'input> {
    type Node = YarnSpinnerParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'a, 'input: 'a> YarnSpinnerParserVisitorCompat<'input> for DeclarationVisitor<'a, 'input> {
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

    fn visit_hashtag(&mut self, ctx: &HashtagContext<'input>) -> Self::Return {
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
            let msg = format!(
                "{} has already been declared in {}, line {}",
                existing_explicit_declaration.name,
                existing_explicit_declaration.source_file_name,
                existing_explicit_declaration.source_file_line(),
            );
            self.diagnostics.push(
                Diagnostic::from_message(msg)
                    .with_file_name(&self.source_file_name)
                    .read_parser_rule_context(ctx),
            );
            return;
        }

        // Figure out the value and its type
        let mut constant_value_visitor =
            ConstantValueVisitor::new(self.source_file_name.clone(), self.diagnostics.clone());
        let value_context = ctx.value().unwrap();
        let value = constant_value_visitor.visit(&*value_context);
        self.diagnostics
            .extend_from_slice(&constant_value_visitor.diagnostics);

        // Did the source code name an explicit type?
        if let Some(declaration_type) = ctx.declaration_type.as_ref() {
            let explicit_type = match self
                .keywords_to_builtin_types
                .get(declaration_type.get_text())
            {
                Some(builtin_type) => builtin_type,

                // The type name provided didn't map to a built-in
                // type. Look for the type in our Types collection.
                None => match self
                    .types
                    .iter()
                    .find(|t| t.to_string() == declaration_type.get_text())
                {
                    Some(explicit_type) => explicit_type,
                    None => {
                        // We didn't find a type by this name.
                        let msg = format!("Unknown type {}", declaration_type.get_text());
                        self.diagnostics.push(
                            Diagnostic::from_message(msg)
                                .with_file_name(&self.source_file_name)
                                .read_parser_rule_context(ctx),
                        );
                        return;
                    }
                },
            };

            // Check that the type we've found is compatible with the
            // type of the value that was provided - if it doesn't,
            // that's a type error
            if !value.r#type.is_sub_type_of(explicit_type) {
                let msg = format!(
                    "Type {} does not match value {} ({})",
                    declaration_type.get_text(),
                    value_context.get_text(),
                    value.r#type
                );
                self.diagnostics.push(
                    Diagnostic::from_message(msg)
                        .with_file_name(&self.source_file_name)
                        .read_parser_rule_context(ctx),
                );
                return;
            }
        }
        // We're done creating the declaration!
        let description = compiler::get_document_comments(self.tokens, ctx);
        let line = variable_context.start().line as usize;
        let declaration = Declaration {
            name: variable_name,
            r#type: value.r#type.clone(),
            default_value: value.internal_value.clone().unwrap(),
            description,
            source_file_name: self.source_file_name.clone().into(),
            source_node_name: self.current_node_name.clone(),
            // All positions are +1 compared to original implementation, but the result is the same.
            // I suspect the C# ANTLR implementation is 1-based while antlr4rust is 0-based.
            range: Position {
                line,
                character: variable_context.start().column as usize + 1,
            }..=Position {
                line,
                character: variable_context.stop().column as usize
                    + 1
                    + variable_context.get_text().len(),
            },
            is_implicit: false,
        };
        self.new_declarations.push(declaration);
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
        let result = compile(CompilationJob {
            files: vec![file],
            library: None,
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        });

        println!("{:?}", result.diagnostics);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.declarations.len(), 4);
        assert_eq!(
            result.declarations[0],
            Declaration {
                name: "$foo".to_string(),
                default_value: 1.0.into(),
                description: "".to_string(),
                source_file_name: DeclarationSource::File("test.yarn".to_string()),
                source_node_name: Some("test".to_string()),
                is_implicit: false,
                r#type: Type::Number(NumberType),
                range: Position {
                    line: 3,
                    character: 11,
                }..=Position {
                    line: 3,
                    character: 15,
                },
            }
        );

        assert_eq!(
            result.declarations[1],
            Declaration {
                name: "$bar".to_string(),
                default_value: "2".to_string().into(),
                description: "".to_string(),
                source_file_name: DeclarationSource::File("test.yarn".to_string()),
                source_node_name: Some("test".to_string()),
                is_implicit: false,
                r#type: Type::String(StringType),
                range: Position {
                    line: 4,
                    character: 11,
                }..=Position {
                    line: 4,
                    character: 15,
                },
            }
        );

        assert_eq!(
            result.declarations[2],
            Declaration {
                name: "$baz".to_string(),
                default_value: true.into(),
                description: "".to_string(),
                source_file_name: DeclarationSource::File("test.yarn".to_string()),
                source_node_name: Some("test".to_string()),
                is_implicit: false,
                r#type: Type::Boolean(BooleanType),
                range: Position {
                    line: 5,
                    character: 11,
                }..=Position {
                    line: 5,
                    character: 15,
                },
            }
        );

        assert_eq!(
            result.declarations[3],
            Declaration {
                name: "$quux".to_string(),
                default_value: "hello there".to_string().into(),
                description: "".to_string(),
                source_file_name: DeclarationSource::File("test.yarn".to_string()),
                source_node_name: Some("test".to_string()),
                is_implicit: false,
                r#type: Type::String(StringType),
                range: Position {
                    line: 6,
                    character: 11,
                }..=Position {
                    line: 6,
                    character: 16,
                },
            }
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
        let result = compile(CompilationJob {
            files: vec![file],
            library: None,
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        });

        assert!(result.declarations.is_empty());

        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(
            result.diagnostics[0],
            Diagnostic::from_message("Type string does not match value 1 (Number)".to_string())
                .with_file_name("test.yarn".to_string())
                .with_context("<<declare $foo to 1 as string>>")
                .with_range(
                    Position {
                        line: 3,
                        character: 1,
                    }..=Position {
                        line: 3,
                        character: 30,
                    }
                )
                .with_severity(DiagnosticSeverity::Error)
        );
    }
}
