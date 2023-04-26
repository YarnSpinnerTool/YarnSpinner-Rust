//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/YarnSpinnerRuleContextExt.cs>

use crate::prelude::generated::yarnspinnerparser::YarnSpinnerParserContext;
use antlr_rust::interval_set::Interval;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::ParseTree;

pub trait ParserRuleContextExt<'input>: ParserRuleContext<'input> {
    /// Returns the original text of this [`ParserRuleContext`], including all
    /// whitespace.
    ///
    /// ## Implementation Notes
    ///
    /// Does not include comments because antlr4rust does not let us access the
    /// raw `CharStream` from a token, and passing that around would make the API explode.
    fn get_text_with_whitespace(&self) -> String {
        // The implementation is massively different from the original C# code.
        let start = self.start().get_start() as usize;
        let stop = self.stop().get_stop() as usize;
        let line = self.get_line_recursively();
        println!("line: {:?}", line);
        let len = line.len();
        println!("len: {:?}", len);
        println!("start: {:?}", start);
        println!("stop: {:?}", stop);
        line[(start as usize)..(stop as usize)].to_string()
    }

    fn get_line_recursively(&self) -> String {
        if self.get_children().next().is_none() {
            self.get_text()
        } else {
            self.get_children()
                .map(|c| c.get_line_recursively())
                .collect::<Vec<_>>()
                .join("")
        }
    }
}

impl<'input, T: ?Sized> ParserRuleContextExt<'input> for T where T: ParserRuleContext<'input> {}
