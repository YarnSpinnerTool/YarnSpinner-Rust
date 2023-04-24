//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ErrorStrategy.cs>

use antlr_rust::errors::{ANTLRError, InputMisMatchError, NoViableAltError};
use antlr_rust::parser::ParserNodeType;
use antlr_rust::token::Token;
use antlr_rust::token_factory::TokenFactory;
use std::rc::Rc;

use crate::prelude::generated::yarnspinnerparser;

use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::rule_context::CustomRuleContext;
use antlr_rust::tree::Tree;
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

    /// ## Implementation notes
    /// The implementation of `DefaultErrorStrategy` will run as well, right after this.
    /// This means that behaviors overridden in the original implementation will instead be handled twice here.
    /// This is alright, since this amounts simply printing an unspecific error message after the specific one.
    /// The reason we have to do this is because the API to tell the `DefaultErrorStrategy` that we are currently
    /// in error recovery mode is private, so we can only access it indirectly by handling something.
    fn report_error(&mut self, recognizer: &mut T, e: &ANTLRError) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        let msg = match e {
            ANTLRError::NoAltError(e) => self.report_no_viable_alternative(recognizer, e),
            ANTLRError::InputMismatchError(e) => self.report_input_mismatch(recognizer, e),
            // Already handled by `DefaultErrorStrategy`
            _ => return,
        };
        let offending_token_index = e.get_offending_token().map(|it| it.get_token_index());
        recognizer.notify_error_listeners(msg, offending_token_index, Some(e));
        self.default_error_strategy.report_error(recognizer, e);
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
        if is_inside_rule(recognizer, yarnspinnerparser::RULE_if_statement)
            && recognizer.get_parser_rule_context().get_rule_index()
                == yarnspinnerparser::RULE_statement
            && e.start_token.token_type == yarnspinnerparser::COMMAND_START
            && e.base.offending_token.token_type == yarnspinnerparser::COMMAND_ELSE
        {
            // We are inside an if statement, we're attempting to parse a
            // statement, and we got an '<<', 'else', and we weren't able
            // to match that. The programmer included an extra '<<else>>'.
            "More than one <<else>> statement in an <<if>> statement isn't allowed".to_owned()
        } else if e.start_token.token_type == yarnspinnerparser::COMMAND_START
            && e.base.offending_token.token_type == yarnspinnerparser::COMMAND_END
        {
            // We saw a << immediately followed by a >>. The programmer
            // forgot to include command text.
            "You forgot to include command text between << and >>".to_owned()
        } else {
            let rule_context = recognizer.get_parser_rule_context();
            format!(
                "Unexpected \"{}\" while reading {}",
                e.base.offending_token.get_text(),
                Self::get_friendly_name_for_rule_context(rule_context)
            )
        }
    }

    fn report_input_mismatch<T: Parser<'input, Node = Ctx, TF = Ctx::TF>>(
        &self,
        recognizer: &mut T,
        e: &InputMisMatchError,
    ) -> String {
        let rule_context = recognizer.get_parser_rule_context();
        let msg = match rule_context.get_rule_index() {
            yarnspinnerparser::RULE_if_statement => {
                match e.base.offending_token.token_type {
                    yarnspinnerparser::BODY_END => {
                        // We have exited a body in the middle of an if
                        // statement. The programmer forgot to include an
                        // <<endif>>.
                        Some(format!(
                            "Expected an <<endif>> to match the <<if>> statement on line {}",
                            rule_context.start().get_line()
                        ))
                    }
                    yarnspinnerparser::COMMAND_ELSE
                        if recognizer
                            .get_expected_tokens()
                            .contains(yarnspinnerparser::COMMAND_ENDIF) =>
                    {
                        // We saw an else, but we expected to see an endif. The
                        // programmer wrote an additional <<else>>.
                        Some(
                            "More than one <<else>> statement in an <<if>> statement isn't allowed"
                                .to_owned(),
                        )
                    }
                    _ => None,
                }
            }
            yarnspinnerparser::RULE_variable
                if e.base.offending_token.token_type == yarnspinnerparser::FUNC_ID =>
            {
                // We're parsing a variable (which starts with a '$'),
                // but we encountered a FUNC_ID (which doesn't). The
                // programmer forgot to include the '$'.
                Some("Variables must start with a '$'".to_owned())
            }
            _ => None,
        };

        msg.unwrap_or_else(|| {
            format!(
                "Unexpected \"{}\" while reading {}",
                e.base.offending_token.get_text(),
                Self::get_friendly_name_for_rule_context_with_article(rule_context)
            )
        })
    }

    fn get_friendly_name_for_rule_context(ctx: &Rc<Ctx::Type>) -> String {
        let rule_name = yarnspinnerparser::ruleNames[ctx.get_rule_index()];
        rule_name.replace('_', " ")
    }

    fn get_friendly_name_for_rule_context_with_article(ctx: &Rc<Ctx::Type>) -> String {
        let friendly_name = Self::get_friendly_name_for_rule_context(ctx);
        // If the friendly name's first character is a vowel, the
        // article is 'an'; otherwise, 'a'.
        let first_letter = friendly_name.chars().next().unwrap();
        let article = if "aeiou".contains(first_letter) {
            "an"
        } else {
            "a"
        };
        format!("{} {}", article, friendly_name)
    }
}

fn is_inside_rule<'a>(recognizer: &impl Parser<'a>, rule_index: usize) -> bool {
    let mut current_context = Some(recognizer.get_parser_rule_context().clone());
    while let Some(context) = current_context {
        if context.get_rule_index() == rule_index {
            return true;
        }
        current_context = context.get_parent();
    }
    false
}
