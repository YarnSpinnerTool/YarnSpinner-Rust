use crate::parser_rule_context_ext::{ParserRuleContextExt, ParserRuleContextExtRangeSource};
use crate::prelude::*;
use antlr_rust::rule_context::CustomRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_factory::TokenFactory;
use yarn_slinger_core::prelude::*;

pub(crate) trait DiagnosticExt {
    fn with_parser_context<'input, T>(
        self,
        ctx: &T,
        token_stream: &ActualTokenStream<'input>,
    ) -> Self
        where
            T: ParserRuleContextExt<'input>,
            <<<<T as CustomRuleContext<'input>>::TF as TokenFactory<'input>>::Inner as Token>::Data as ToOwned>::Owned:
            Into<String>
    ;
}

impl DiagnosticExt for Diagnostic {
    fn with_parser_context<'input, T>(
        self,
        ctx: &T,
        token_stream: &ActualTokenStream<'input>,
    ) -> Self
        where
            T: ParserRuleContextExt<'input>,
            <<<<T as CustomRuleContext<'input>>::TF as TokenFactory<'input>>::Inner as Token>::Data as ToOwned>::Owned:
            Into<String>
    {
        let lines_above_and_below_offending_line = 2;
        let lines_around = ctx.get_lines_around(token_stream, lines_above_and_below_offending_line);
        let range = ctx.range();
        self.with_range(range)
            .with_context(lines_around.lines)
            .with_start_line(lines_around.first_line)
    }
}
