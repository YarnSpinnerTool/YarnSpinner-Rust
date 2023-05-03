//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/YarnSpinnerRuleContextExt.cs>

use crate::prelude::*;
use antlr_rust::int_stream::IntStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_stream::TokenStream;

pub(crate) trait ParserRuleContextExt<'input>: ParserRuleContext<'input> {
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
            // ## Implementation Notes
            // Uses `get_token_index()` instead of `get_start()` and `get_stop()`.
            // I suspect the `get_text_from_interval` implementation behaves differently
            // from the C# ANTLR runtime. Might even be bugged. Alas, the way this
            // function is written now behaves the same way the original did, even if it does not seem so.
            token_stream.get_text_from_interval(start, stop)
        }
    }

    fn get_lines_around(&self, token_stream: &ActualTokenStream<'input>) -> LinesAroundResult {
        let start = self.start().get_token_index();
        let stop = self.stop().get_token_index();
        let first_line = self.start().get_line() as usize;

        let head = token_stream.get_text_from_interval(0, start - 1);
        let body = token_stream.get_text_from_interval(start, stop);
        let tail = token_stream.get_text_from_interval(stop + 1, token_stream.size() - 1);
        let head_start_index = head.rfind('\n').map(|index| index + 1).unwrap_or(0);

        let (head_start_index, first_line) = (0..2).fold(
            (head_start_index, first_line),
            |(head_start_index, first_line), _| {
                let head = &head[..head_start_index - 1];
                if let Some(head_start_index) = head.rfind('\n').map(|index| index + 1) {
                    (head_start_index, first_line - 1)
                } else {
                    (0, first_line)
                }
            },
        );
        let tail_end_index = tail.find('\n').unwrap_or(tail.len());
        let tail_end_index = (0..2).fold(tail_end_index, |tail_end_index, _| {
            let tail = &tail[tail_end_index + 1..];
            if let Some(tail_end_index) = tail.find('\n') {
                tail_end_index
            } else {
                tail.len()
            }
        });

        let head = &head[head_start_index..];
        let tail = &tail[..tail_end_index];
        LinesAroundResult {
            lines: format!("{head}{body}{tail}"),
            first_line,
        }
    }
}

pub(crate) struct LinesAroundResult {
    pub(crate) lines: String,
    pub(crate) first_line: usize,
}

impl<'input, T: ?Sized> ParserRuleContextExt<'input> for T where T: ParserRuleContext<'input> {}
