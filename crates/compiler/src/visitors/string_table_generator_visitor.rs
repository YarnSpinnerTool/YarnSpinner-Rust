//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/StringTableGeneratorVisitor.cs>
use crate::compiler;
use crate::prelude::generated::{yarnspinnerparser::*, yarnspinnerparservisitor::*};
use crate::prelude::*;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, Tree};
use better_any::TidExt;
use std::rc::Rc;

#[derive(Clone)]
/// A Visitor that walks an expression parse tree and generates string
/// table entries, which are provided to a [`StringTableManager`].
/// This string table can then be provided
/// to future compilation passes, or stored for later use. Call the
/// [`visit`] method to begin generating string table entries.
pub(crate) struct StringTableGeneratorVisitor<'a, 'input: 'a> {
    pub(crate) diagnostics: Vec<Diagnostic>,
    current_node_name: String,
    file_name: String,
    pub(crate) string_table_manager: StringTableManager,
    tokens: &'a ActualTokenStream<'input>,
    _dummy: (),
}

impl<'a, 'input: 'a> StringTableGeneratorVisitor<'a, 'input> {
    pub(crate) fn new(
        file_name: String,
        string_table_manager: StringTableManager,
        tokens: &'a ActualTokenStream<'input>,
    ) -> Self {
        Self {
            file_name,
            string_table_manager,
            diagnostics: Default::default(),
            current_node_name: Default::default(),
            tokens,
            _dummy: (),
        }
    }
}

impl<'a, 'input: 'a> ParseTreeVisitorCompat<'input> for StringTableGeneratorVisitor<'a, 'input> {
    type Node = YarnSpinnerParserContextType;

    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'a, 'input: 'a> YarnSpinnerParserVisitorCompat<'input>
    for StringTableGeneratorVisitor<'a, 'input>
{
    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        let parent = ctx.get_parent().unwrap();
        for child in parent.get_children() {
            if let Some(value_context_all) = child.downcast_ref::<VariableContext>() {
                panic!("here!");
            }
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
    composed_string
}

fn get_hashtag_texts(hashtags: &[Rc<HashtagContext>]) -> Vec<String> {
    hashtags
        .iter()
        .map(|t| {
            t.text
                .as_ref()
                .expect("No text in hashtag")
                .get_text()
                .to_owned()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::InputStream;

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
        let lexer = YarnSpinnerLexer::new(InputStream::new(input));
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
a {$foo} cool expression
==="
            .to_string(),
        };
        let result = compile(CompilationJob {
            files: vec![file],
            library: None,
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        });
        let string_table = result.string_table;
        assert_eq!(string_table.len(), 3);
        assert_eq!(
            string_table["line:test.yarn-test-0"],
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
            string_table["line:test.yarn-test-1"],
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
            string_table["line:test.yarn-test-2"],
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
        let result = compile(CompilationJob {
            files: vec![file],
            library: None,
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        });
        assert!(result.program.is_none());
        let diagnostics = result.diagnostics;
        assert_eq!(diagnostics.len(), 2);

        let range = Position {
            line: 5,
            character: 8,
        }..=Position {
            line: 5,
            character: 9,
        };
        let context = "a {very} cool expression\n       ^".to_owned();
        let first_expected =
            Diagnostic::from_message("Unexpected \"}\" while reading a function call".to_string())
                .with_file_name("test.yarn".to_string())
                .with_range(range.clone())
                .with_context(context.clone())
                .with_severity(DiagnosticSeverity::Error);

        let second_expected =
            Diagnostic::from_message("mismatched input '}' expecting '('".to_string())
                .with_file_name("test.yarn".to_string())
                .with_range(range)
                .with_context(context)
                .with_severity(DiagnosticSeverity::Error);
        if diagnostics[0] == first_expected {
            assert_eq!(diagnostics[1], second_expected);
        } else {
            assert_eq!(diagnostics[0], second_expected);
            assert_eq!(diagnostics[1], first_expected);
        }
    }
}
