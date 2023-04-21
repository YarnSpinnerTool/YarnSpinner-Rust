//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/StringTableGeneratorVisitor.cs>
use crate::compiler;
use crate::prelude::generated::{
    yarnspinnerparser, yarnspinnerparser::*, yarnspinnerparservisitor::*,
};
use crate::prelude::*;
use antlr_rust::parser_rule_context::{BaseParserRuleContext, DerefSeal, ParserRuleContext};
use antlr_rust::token::Token;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};
use antlr_rust::{TidAble, TidExt};
use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
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

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        todo!()
    }
}

impl<'input> YarnSpinnerParserVisitorCompat<'input> for StringTableGeneratorVisitor {
    fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) -> Self::Return {
        let line_number = ctx.start().line;
        let hashtags = ctx.hashtag_all();
        let line_id_tag = get_line_id_tag(&hashtags);
        let line_id = line_id_tag.as_ref().and_then(|t| t.text.as_ref());

        let hashtag_texts = get_hashtag_texts(&hashtags);

        let children = ctx.line_formatted_text().unwrap().get_children();

        let composed_string = generate_formatted_text(children);

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

fn generate_formatted_text<'a>(nodes: impl Iterator<Item = impl Tree>) -> String {
    let mut expression_count = 0;
    let mut composed_string = String::new();
    // First, visit all of the nodes, which are either terminal
    // text nodes or expressions. if they're expressions, we
    // evaluate them, and inject a positional reference into the
    // final string.
    for node in nodes {
        if node.is::<TerminalNode<usize>>() {
            composed_string.push_str(&node.get_text());
        } else if node.is::<impl ParserRuleContext>() {
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
        .map(|t| t.text.expect("No text in hashtag").get_text().to_owned())
        .collect()
}
