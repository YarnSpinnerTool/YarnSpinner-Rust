//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ErrorStrategy.cs>

use crate::prelude::generated::yarnspinnerparser::YarnSpinnerParserContextType;
use antlr_rust::errors::ANTLRError;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::{DefaultErrorStrategy, ErrorStrategy as AntlrErrorStrategy, Parser};

pub struct ErrorStrategy<'input, Ctx: ParserNodeType<'input>> {
    default_error_strategy: DefaultErrorStrategy<'input, Ctx>,
}

impl<'input, Ctx: ParserNodeType<'input>> ErrorStrategy<'input, Ctx> {
    pub(crate) fn new() -> Self {
        Self {
            default_error_strategy: DefaultErrorStrategy::new(),
        }
    }
}

better_any::tid! { impl<'i,Ctx> TidAble<'i> for ErrorStrategy<'i,Ctx> where Ctx: ParserNodeType<'i>}

impl<'input, T: Parser<'input>> AntlrErrorStrategy<'input, T> for ErrorStrategy<'input, T::Node> {
    fn reset(&mut self, recognizer: &mut T) {
        self.default_error_strategy.reset(recognizer);
    }

    fn recover_inline(
        &mut self,
        recognizer: &mut T,
    ) -> Result<<T::TF as TokenFactory<'input>>::Tok, ANTLRError> {
        self.default_error_strategy.recover_inline(recognizer)
    }

    fn recover(&mut self, recognizer: &mut T, e: &ANTLRError) -> Result<(), ANTLRError> {
        self.default_error_strategy.recover(recognizer, e)
    }

    fn sync(&mut self, recognizer: &mut T) -> Result<(), ANTLRError> {
        self.default_error_strategy.sync(recognizer)
    }

    fn in_error_recovery_mode(&mut self, recognizer: &mut T) -> bool {
        self.default_error_strategy
            .in_error_recovery_mode(recognizer)
    }

    fn report_error(&mut self, recognizer: &mut T, e: &ANTLRError) {
        self.default_error_strategy.report_error(recognizer, e)
    }

    fn report_match(&mut self, recognizer: &mut T) {
        self.default_error_strategy.report_match(recognizer)
    }
}
