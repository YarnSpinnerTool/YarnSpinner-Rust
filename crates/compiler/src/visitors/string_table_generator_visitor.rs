//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/StringTableGeneratorVisitor.cs>
use crate::parser::generated::yarnspinnerparserlistener::YarnSpinnerParserListener;
use crate::prelude::generated::{yarnspinnerparser::*, yarnspinnerparservisitor::*};
use crate::prelude::*;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeListener, ParseTreeVisitorCompat, TerminalNode, Tree};
use antlr_rust::TidExt;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
/// A Visitor that walks an expression parse tree and generates string
/// table entries, which are provided to a [`StringTableManager`].
/// This string table can then be provided
/// to future compilation passes, or stored for later use. Call the
/// [`visit`] method to begin generating string table entries.
pub(crate) struct StringTableGeneratorVisitor {
    diagnostics: Vec<Diagnostic>,
    current_node_name: String,
    file_name: String,
    string_table_manager: StringTableManager,
}

impl StringTableGeneratorVisitor {
    pub(crate) fn new(file_name: String, string_table_manager: StringTableManager) -> Self {
        Self {
            file_name,
            string_table_manager,
            diagnostics: Default::default(),
            current_node_name: Default::default(),
        }
    }
}

impl ParseTreeVisitorCompat<'_> for StringTableGeneratorVisitor {
    type Node = YarnSpinnerParserContextType;

    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        Box::leak(Box::new(()))
    }
}

impl<'input> YarnSpinnerParserVisitorCompat<'input> for StringTableGeneratorVisitor {
    fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) -> Self::Return {
        let line_number = ctx.start().line;
        let hashtags = ctx.hashtag_all();
        let line_id_tag = get_line_id_tag(&hashtags);
        let line_id = line_id_tag.as_ref().and_then(|t| t.text.as_ref());

        let hashtag_texts = get_hashtag_texts(&hashtags);

        let line_formatted_text = ctx.line_formatted_text().unwrap();
        let composed_string = generate_formatted_text(line_formatted_text.get_children());

        if let Some(line_id) = line_id {
            if self.string_table_manager.contains_key(&line_id.to_string()) {
                // TODO: Duplicate line ID, add to diagnostics
            }
        };

        // TODO
        self.string_table_manager.insert(
            composed_string,
            StringInfo {
                text: todo!(),
                node_name: todo!(),
                line_number: todo!(),
                file_name: todo!(),
                metadata: todo!(),
                ..Default::default()
            },
        );

        /*




               string stringID = stringTableManager.RegisterString(
                   composedString.ToString(),
                   fileName,
                   currentNodeName,
                   lineID,
                   lineNumber,
                   hashtagText);

               if (lineID == null)
               {
                   var hashtag = new YarnSpinnerParser.HashtagContext(context, 0);
                   hashtag.text = new CommonToken(YarnSpinnerLexer.HASHTAG_TEXT, stringID);
                   context.AddChild(hashtag);
               }

               return 0;
        */
        self.visit_children(ctx);
    }
}

#[derive(Debug, Clone, Default)]
struct FormattedTextVisitor(String);

impl ParseTreeVisitorCompat<'_> for FormattedTextVisitor {
    type Node = YarnSpinnerParserContextType;
    type Return = String;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        aggregate + &next
    }
}

impl<'input> YarnSpinnerParserVisitorCompat<'input> for FormattedTextVisitor {
    fn visit_dialogue(&mut self, ctx: &DialogueContext<'input>) -> Self::Return {
        /// In theory, this is nearly the default impl and should be replaced by a simple overload of `visit_line_formatted_text`, however,
        /// this ends up resulting in a stack overflow :(
        let mut result = Self::Return::default();
        for node in ctx.node_all() {
            let body = node.body().unwrap();
            for statement in body.statement_all() {
                if !self.should_visit_next_child(&*statement, &result) {
                    break;
                }
                let line_statement = statement.line_statement().unwrap();
                let current = self.visit(&*line_statement);
                result = self.aggregate_results(result, current);
            }
        }
        result
    }

    fn visit_line_formatted_text(
        &mut self,
        ctx: &Line_formatted_textContext<'input>,
    ) -> Self::Return {
        for child in ctx.get_children() {
            if child.get_child_count() == 0 {
                println!("leaf: {}", child.get_text())
            } else {
                println!("edge: {}", child.get_text())
            }
        }
        if let Some(text) = ctx.TEXT(0) {
            println!("Visiting line formatted text: {}", text.get_text());
            text.get_text()
        } else if let Some(expression) = ctx.expression(0) {
            println!("Visiting expression: :{}", expression.get_text());
            expression.get_text()
        } else {
            panic!("Unexpected line formatted text")
        }
    }
}

/// Takes a string like
/// `Hi there { some_expression }, how are you { another_expression } doing?`
/// and turns it into
/// `Hi there {0}, how are you {1}? doing`
fn generate_formatted_text<'a>(
    nodes: impl Iterator<Item = Rc<impl YarnSpinnerParserContext<'a> + ?Sized>>,
) -> String {
    let mut expression_count = 0;
    let mut composed_string = String::new();
    // First, visit all of the nodes, which are either terminal
    // text nodes or expressions. if they're expressions, we
    // evaluate them, and inject a positional reference into the
    // final string.
    for node in nodes {
        if node.get_token(TEXT, 0).is_some() {
            composed_string.push_str(&node.get_text());
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
    use crate::parser::generated::yarnspinnerlexer::YarnSpinnerLexer;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::parser_rule_context::RuleContextExt;
    use antlr_rust::tree::ParseTreeVisitor;
    use antlr_rust::InputStream;

    #[test]
    fn ignores_lines_without_expression() {
        let input = "title: Title
---
A line
===
";
        let lexer = YarnSpinnerLexer::new(InputStream::new(input.into()));
        let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));

        let dialogue = parser.dialogue().unwrap();

        let result = FormattedTextVisitor::default().visit(&*dialogue);
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
        let lexer = YarnSpinnerLexer::new(InputStream::new(input.into()));
        let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));

        let dialogue = parser.dialogue().unwrap();

        let result = FormattedTextVisitor::default().visit(&*dialogue);
        let expected = "A line with a {0} expression";
        assert_eq!(result, expected);
    }
}
