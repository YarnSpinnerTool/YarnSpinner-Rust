//! Contains the actual types used in the real-world parser without all the generic type parameters.

use crate::error_strategy::ErrorStrategy;
use crate::prelude::*;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::InputStream;
use generated::yarnspinnerparser::*;

pub type ActualInputStream<'input> = InputStream<&'input [u8]>;
pub type ActualYarnSpinnerLexer<'input> = YarnSpinnerLexer<'input, ActualInputStream<'input>>;
pub type ActualErrorStrategy<'input> = ErrorStrategy<'input, YarnSpinnerParserContextType>;
pub type ActualTokenStream<'input> = CommonTokenStream<'input, ActualYarnSpinnerLexer<'input>>;
pub type ActualYarnSpinnerParser<'input> =
    YarnSpinnerParser<'input, ActualTokenStream<'input>, ActualErrorStrategy<'input>>;
pub type ActualParserContext<'input> = dyn YarnSpinnerParserContext<
    'input,
    Ctx = YarnSpinnerParserContextType,
    TF = LocalTokenFactory<'input>,
>;

pub type ActualRuleContext<'input> = dyn ParserRuleContext<
    'input,
    Ctx = YarnSpinnerParserContextType,
    TF = LocalTokenFactory<'input>,
>;
