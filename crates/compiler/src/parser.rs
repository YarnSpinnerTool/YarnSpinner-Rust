//! The parser for the compiler.

pub mod generated;
mod indent_aware_lexer;

use crate::error_strategy::ErrorStrategy;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use generated::yarnspinnerparser::YarnSpinnerParser;
use generated::yarnspinnerparser::YarnSpinnerParserContextType;
pub use indent_aware_lexer::IndentAwareYarnSpinnerLexer as YarnSpinnerLexer;

pub type ActualInputStream<'input> = InputStream<&'input [u8]>;
pub type ActualYarnSpinnerLexer<'input> = YarnSpinnerLexer<'input, ActualInputStream<'input>>;
pub type ActualErrorStrategy<'input> = ErrorStrategy<'input, YarnSpinnerParserContextType>;
pub type ActualTokenStream<'input> = CommonTokenStream<'input, ActualYarnSpinnerLexer<'input>>;
pub type ActualYarnSpinnerParser<'input> =
    YarnSpinnerParser<'input, ActualTokenStream<'input>, ActualErrorStrategy<'input>>;
