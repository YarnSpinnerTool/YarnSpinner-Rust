//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/StringTableGeneratorVisitor.cs>
use crate::prelude::generated::{yarnspinnerparser::*, yarnspinnerparservisitor::*};
use crate::prelude::*;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ParseTreeVisitorCompat, Tree};

/// Represents StringTableGeneratorVisitor
struct StringTableGeneratorVisitor(StringTableManager);

impl<'input> ParseTreeVisitorCompat<'input> for StringTableGeneratorVisitor {
    type Node = YarnSpinnerParserContextType;

    type Return = StringTableManager;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
    }
}

impl<'input> YarnSpinnerParserVisitor<'input> for StringTableGeneratorVisitor {
    /// VisitLine_statement of StringTableGeneratorVisitor
    fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) {
        let line_number = ctx.start().line;
        let hashtags = ctx.hashtag_all();
        let line_id_tag: Option<HashtagContext> = todo!();
        let line_id = line_id_tag.as_ref().and_then(|t| t.text.as_ref());

        let hashtag_texts = hashtags.iter().map(|t| t.text.as_ref());

        let generate_formatted_text = |_| (todo!(), todo!());
        let (composed_string, expression_count) =
            generate_formatted_text(ctx.line_formatted_text().unwrap().get_children());

        if let Some(line_id) = line_id {
            if self.0.contains_key(&line_id.to_string()) {
                // TODO: Duplicate line ID, add to diagnostics
            }
        };

        // TODO
        self.0.insert(
            composed_string,
            StringInfo {
                text: (),
                node_name: (),
                line_number: (),
                file_name: (),
                metadata: (),
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
