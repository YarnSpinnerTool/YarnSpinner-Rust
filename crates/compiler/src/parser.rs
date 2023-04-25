//! The parser for the compiler.

pub mod generated;
mod indent_aware_lexer;

pub use indent_aware_lexer::IndentAwareYarnSpinnerLexer as YarnSpinnerLexer;
