//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/YarnSpinnerRuleContextExt.cs>

use crate::prelude::*;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_stream::TokenStream;

pub trait ParserRuleContextExt<'input>: ParserRuleContext<'input> {
    /// Returns the original text of this [`ParserRuleContext`], including all
    /// whitespace.
    ///
    /// ## Implementation Notes
    ///
    /// In contrast to the original, we need to pass a token stream here, because
    /// antlr4rust does not allow us to retrieve it from the context or a token.
    fn get_text_with_whitespace(&self, token_stream: &ActualTokenStream<'input>) -> String {
        // We can't use "expressionContext.GetText()" here, because
        // that just concatenates the text of all captured tokens,
        // and doesn't include text on hidden channels (e.g.
        // whitespace and comments).

        // some times it seems that vscode can request a negative interval
        // almost certainly something wrong we are doing
        // but as a non-crashing fallback we prevent this
        let start = self.start().get_token_index();
        let stop = self.stop().get_token_index();
        if start > stop {
            self.get_text()
        } else {
            token_stream.get_text_from_interval(start, stop)
        }
    }
}

impl<'input, T: ?Sized> ParserRuleContextExt<'input> for T where T: ParserRuleContext<'input> {}
