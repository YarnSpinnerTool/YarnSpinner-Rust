//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ErrorStrategy.cs>

use antlr_rust::errors::{ANTLRError, FailedPredicateError, InputMisMatchError, NoViableAltError};
use antlr_rust::parser::ParserNodeType;

use antlr_rust::rule_context::CustomRuleContext;
use antlr_rust::token::Token;
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

/// Rust has no inheritance, so we cannot inherit from `DefaultErrorStrategy` and override the methods we want to change.
/// Instead, we have to copy some parts of antlr4rust implementation and relay some calls to the proxied `default_error_strategy`.
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
        if self
            .default_error_strategy
            .in_error_recovery_mode(recognizer)
        {
            return;
        }

        //self.begin_error_condition(recognizer);
        let msg = match e {
            ANTLRError::NoAltError(e) => self.report_no_viable_alternative(recognizer, e),
            ANTLRError::InputMismatchError(e) => self.report_input_mismatch(recognizer, e),
            ANTLRError::PredicateError(e) => self.report_failed_predicate(recognizer, e),
            _ => e.to_string(),
        };
        let offending_token_index = e.get_offending_token().map(|it| it.get_token_index());
        recognizer.notify_error_listeners(msg, offending_token_index, Some(&e))
    }

    fn report_match(&mut self, recognizer: &mut T) {
        self.default_error_strategy.report_match(recognizer)
    }
}

impl<'input, Ctx: ParserNodeType<'input>> ErrorStrategy<'input, Ctx> {
    fn report_no_viable_alternative<T: Parser<'input, Node = Ctx, TF = Ctx::TF>>(
        &self,
        recognizer: &mut T,
        e: &NoViableAltError,
    ) -> String {
        String::from("no viable alternative")
    }

    fn report_input_mismatch<T: Parser<'input, Node = Ctx, TF = Ctx::TF>>(
        &self,
        recognizer: &mut T,
        e: &InputMisMatchError,
    ) -> String {
        String::from("input mismatch")
    }

    /// Implementation copy-pasted from `DefaultErrorStrategy::report_failed_predicate` because the function in question is private.
    fn report_failed_predicate<T: Parser<'input, Node = Ctx, TF = Ctx::TF>>(
        &self,
        recognizer: &mut T,
        e: &FailedPredicateError,
    ) -> String {
        format!(
            "rule {} {}",
            recognizer.get_rule_names()[recognizer.get_parser_rule_context().get_rule_index()],
            e.base.message
        )
    }
}
