//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/StringTableGeneratorVisitor.cs>
use crate::prelude::generated::{yarnspinnerparser::*, yarnspinnerparservisitor::*};
use crate::prelude::*;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, Tree};
use std::rc::Rc;

#[derive(Clone)]
/// A Visitor that walks an expression parse tree and generates string
/// table entries, which are provided to a [`StringTableManager`].
/// This string table can then be provided
/// to future compilation passes, or stored for later use. Call the
/// [`visit`] method to begin generating string table entries.
pub(crate) struct StringTableGeneratorVisitor<'input> {
    pub(crate) diagnostics: Vec<Diagnostic>,
    current_node_name: String,
    pub(crate) string_table_manager: StringTableManager,
    file: FileParseResult<'input>,
    _dummy: (),
}

impl<'input> StringTableGeneratorVisitor<'input> {
    pub(crate) fn new(
        string_table_manager: StringTableManager,
        file: FileParseResult<'input>,
    ) -> Self {
        Self {
            file,
            string_table_manager,
            diagnostics: Default::default(),
            current_node_name: Default::default(),
            _dummy: (),
        }
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for StringTableGeneratorVisitor<'input> {
    type Node = YarnSpinnerParserContextType;

    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'input> YarnSpinnerParserVisitorCompat<'input> for StringTableGeneratorVisitor<'input> {
    fn visit_node(&mut self, ctx: &NodeContext<'input>) -> Self::Return {
        let mut tags = Vec::new();
        for header in ctx.header_all() {
            let header_key = header.header_key.as_ref().unwrap().get_text();
            if header_key == "title" {
                header
                    .header_value
                    .as_ref()
                    .unwrap()
                    .get_text()
                    .clone_into(&mut self.current_node_name)
            } else if header_key == "tags" {
                let header_value = header
                    .header_value
                    .as_ref()
                    .map(|header| header.get_text())
                    .unwrap_or_default();
                // Split the list of tags by spaces, and use that
                tags = header_value
                    .split_whitespace()
                    .map(ToOwned::to_owned)
                    .collect();
            }
        }
        if !self.current_node_name.is_empty() && tags.contains(&"rawText".to_owned()) {
            // This is a raw text node. Use its entire contents as a
            // string and don't use its contents.
            let line_id = get_line_id_for_node_name(&self.current_node_name);
            self.string_table_manager.insert(
                line_id,
                StringInfo {
                    text: ctx.body().unwrap().get_text(),
                    node_name: self.current_node_name.clone(),
                    line_number: ctx.body().unwrap().start().line as usize,
                    file_name: self.file.name.clone(),
                    ..Default::default()
                },
            );
        } else {
            // This is a regular node
            // String table generator: don't crash if a node has no body
            if let Some(body) = ctx.body() {
                self.visit(body.as_ref());
            }
        }
    }

    fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) -> Self::Return {
        let hashtags = ctx.hashtag_all();
        let line_id_tag = get_line_id_tag(&hashtags);
        let line_id = line_id_tag.as_ref().and_then(|t| t.text.as_ref());

        if let Some(line_id) = line_id
            && self
                .string_table_manager
                .contains_key(&line_id.get_text().into())
        {
            // The original has a fallback for when this is `null` / `None`,
            // but this can logically not be the case in this scope.
            let diagnostic_context = line_id_tag.clone().unwrap();
            let line_id = line_id.get_text();
            self.diagnostics.push(
                Diagnostic::from_message(format!("Duplicate line ID {line_id}"))
                    .with_parser_context(diagnostic_context.as_ref(), self.file.tokens())
                    .with_file_name(&self.file.name),
            );
            return;
        };

        let line_number = ctx.start().get_line_as_usize();
        let hashtag_texts = get_hashtag_texts(&hashtags);

        let composed_string = generate_formatted_text(&ctx.line_formatted_text().unwrap());

        let string_id = self.string_table_manager.insert(
            line_id.map(|t| t.get_text().into()),
            StringInfo {
                text: composed_string,
                node_name: self.current_node_name.clone(),
                line_number,
                file_name: self.file.name.clone(),
                metadata: hashtag_texts,
                ..Default::default()
            },
        );

        if line_id.is_none() {
            add_hashtag_child(ctx, string_id.0);
        }
    }
}

/// Takes a string like
/// `Hi there { some_expression }, how are you { another_expression } doing?`
/// and turns it into
/// `Hi there {0}, how are you {1}? doing`
fn generate_formatted_text(ctx: &Line_formatted_textContext) -> String {
    let mut expression_count = 0;
    let mut composed_string = String::new();
    // First, visit all of the nodes, which are either terminal
    // text nodes or expressions. if they're expressions, we
    // evaluate them, and inject a positional reference into the
    // final string.
    for child in ctx.get_children() {
        if child.get_child_count() == 0 {
            composed_string.push_str(&child.get_text());
        } else {
            // Expressions in the final string are denoted as the
            // index of the expression, surrounded by braces { }.
            // However, we don't need to write the braces here
            // ourselves, because the text itself that the parser
            // captured already has them. So, we just need to write
            // the expression count.
            composed_string.push_str(&expression_count.to_string());
            expression_count += 1;
        }
    }
    composed_string.trim().to_owned()
}

pub(crate) fn get_hashtag_texts(hashtags: &[Rc<HashtagContext>]) -> Vec<String> {
    hashtags
        .iter()
        .map(|t| {
            t.text
                .as_ref()
                .expect("No text in hashtag")
                .get_text()
                .trim()
                .to_owned()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use antlr_rust::InputStream;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use yarnspinner_core::prelude::Position;

    #[test]
    fn ignores_lines_without_expression() {
        let input = "title: Title
---
A line
===
";
        let result = process_input(input);
        let expected = "A line";
        assert_eq!(result, expected);
    }

    #[test]
    fn formats_lines_with_expression() {
        let input = "title: Title
---
A line with a {$cool} expression
===
";
        let result = process_input(input);
        let expected = "A line with a {0} expression";
        assert_eq!(result, expected);
    }

    #[test]
    fn formats_lines_with_multiple_expressions() {
        let input = "title: Title
---
A line with {$many} many {(1 -(1 * 2))}{$cool} expressions
===
";
        let result = process_input(input);
        let expected = "A line with {0} many {1}{2} expressions";
        assert_eq!(result, expected);
    }

    fn process_input(input: &str) -> String {
        let lexer = YarnSpinnerLexer::new(InputStream::new(input), "input.yarn".to_owned());
        let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));
        let line_formatted_text = parser
            .dialogue()
            .unwrap()
            .node(0)
            .unwrap()
            .body()
            .unwrap()
            .statement(0)
            .unwrap()
            .line_statement()
            .unwrap()
            .line_formatted_text()
            .unwrap();
        generate_formatted_text(&line_formatted_text)
    }

    #[test]
    fn populates_string_table() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
foo
bar
a {1 + 3} cool expression
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

        let string_table = result.string_table;
        assert_eq!(string_table.len(), 3);
        assert_eq!(
            string_table[&"line:test.yarn-test-0".into()],
            StringInfo {
                text: "foo".to_string(),
                node_name: "test".to_string(),
                line_number: 3,
                file_name: "test.yarn".to_string(),
                is_implicit_tag: true,
                metadata: vec![],
            }
        );
        assert_eq!(
            string_table[&"line:test.yarn-test-1".into()],
            StringInfo {
                text: "bar".to_string(),
                node_name: "test".to_string(),
                line_number: 4,
                file_name: "test.yarn".to_string(),
                is_implicit_tag: true,
                metadata: vec![],
            }
        );
        assert_eq!(
            string_table[&"line:test.yarn-test-2".into()],
            StringInfo {
                text: "a {0} cool expression".to_string(),
                node_name: "test".to_string(),
                line_number: 5,
                file_name: "test.yarn".to_string(),
                is_implicit_tag: true,
                metadata: vec![],
            }
        );
    }

    #[test]
    fn catches_expression_errors() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
foo
bar
a {very} cool expression
==="
            .to_string(),
        };
        let result = Compiler {
            files: vec![file],
            library: Default::default(),
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        }
        .compile();

        let diagnostics = result.unwrap_err().0;
        assert_eq!(2, diagnostics.len());

        let range = Position {
            line: 4,
            character: 7,
        }..Position {
            line: 4,
            character: 8,
        };
        let context = "a {very} cool expression\n       ^".to_owned();
        let first_expected =
            Diagnostic::from_message("Unexpected \"}\" while reading a function call".to_string())
                .with_file_name("test.yarn".to_string())
                .with_range(range.clone())
                .with_context(context.clone())
                .with_start_line(4)
                .with_severity(DiagnosticSeverity::Error);

        let second_expected =
            Diagnostic::from_message("mismatched input '}' expecting '('".to_string())
                .with_file_name("test.yarn".to_string())
                .with_range(range)
                .with_context(context)
                .with_start_line(4)
                .with_severity(DiagnosticSeverity::Error);
        if diagnostics[0] == first_expected {
            assert_eq!(diagnostics[1], second_expected);
        } else {
            assert_eq!(diagnostics[0], second_expected);
            assert_eq!(diagnostics[1], first_expected);
        }
    }
}
