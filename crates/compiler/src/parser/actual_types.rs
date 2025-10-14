//! Contains the actual types used in the real-world parser without all the generic type parameters.

use crate::error_strategy::ErrorStrategy;
use crate::prelude::*;
use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use generated::yarnspinnerparser::*;

pub(crate) type ActualInputStream<'input> = InputStream<&'input [u32]>;
pub(crate) type ActualYarnSpinnerLexer<'input> =
    YarnSpinnerLexer<'input, ActualInputStream<'input>>;
pub(crate) type ActualErrorStrategy<'input> = ErrorStrategy<'input, YarnSpinnerParserContextType>;
pub(crate) type ActualTokenStream<'input> =
    CommonTokenStream<'input, ActualYarnSpinnerLexer<'input>>;
pub(crate) type ActualYarnSpinnerParser<'input> =
    YarnSpinnerParser<'input, ActualTokenStream<'input>, ActualErrorStrategy<'input>>;
pub(crate) type ActualParserContext<'input> = dyn YarnSpinnerParserContext<
        'input,
        Ctx = YarnSpinnerParserContextType,
        TF = LocalTokenFactory<'input>,
    >;
