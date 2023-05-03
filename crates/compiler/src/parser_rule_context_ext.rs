//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/YarnSpinnerRuleContextExt.cs>

use crate::prelude::*;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_stream::TokenStream;
use std::iter;

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

    fn get_lines_around(
        &self,
        token_stream: &ActualTokenStream<'input>,
        surrounding_lines: usize,
    ) -> LinesAroundResult {
        // This seems expensive, but it's only used for error reporting.
        let whole_file = token_stream.get_all_text();
        let start = self.start().get_start() as usize;
        let stop = self.stop().get_stop() as usize + 1;
        let first_line = self.start().get_line() as usize;

        let head = &whole_file[..start];
        let body = &whole_file[start..stop];
        let tail = &whole_file[stop..];

        let lines_to_take = surrounding_lines + 1;

        let head_lines = head.lines().rev().take(lines_to_take);
        let head_lines: Vec<_> = if head.ends_with('\n') {
            iter::once("").chain(head_lines).collect()
        } else {
            head_lines.collect()
        };
        let first_line = first_line - head_lines.len();
        let head = head_lines.into_iter().rev().collect::<Vec<_>>().join("\n");

        let tail = tail
            .lines()
            .take(lines_to_take)
            .collect::<Vec<_>>()
            .join("\n");
        let lines = head + &body + &tail;
        LinesAroundResult { lines, first_line }
    }
}

pub(crate) struct LinesAroundResult {
    pub(crate) lines: String,
    pub(crate) first_line: usize,
}

impl<'input, T: ?Sized> ParserRuleContextExt<'input> for T where T: ParserRuleContext<'input> {}
