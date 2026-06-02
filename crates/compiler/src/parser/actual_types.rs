//! Contains the actual types used in the real-world parser without all the generic type parameters.

use crate::prelude::*;
use antlr4rust::InputStream;
use antlr4rust::common_token_stream::CommonTokenStream;
use generated::yarnspinnerparser::*;

pub(crate) type ActualInputStream<'input> = InputStream<&'input [u32]>;
pub(crate) type ActualYarnSpinnerLexer<'input> =
    YarnSpinnerLexer<'input, ActualInputStream<'input>>;
pub(crate) type ActualTokenStream<'input> =
    CommonTokenStream<'input, ActualYarnSpinnerLexer<'input>>;
pub(crate) type ActualYarnSpinnerParser<'input> =
    YarnSpinnerParser<'input, ActualTokenStream<'input>>;
pub(crate) type ActualParserContext<'input> = dyn YarnSpinnerParserContext<
        'input,
        Ctx = YarnSpinnerParserContextType,
        TF = LocalTokenFactory<'input>,
    >;
