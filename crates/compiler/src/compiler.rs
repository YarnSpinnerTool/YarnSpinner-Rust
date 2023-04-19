use crate::output::*;
use crate::parser::generated::yarnspinnerlexer::YarnSpinnerLexer;
use crate::parser::generated::yarnspinnerparser::{
    DialogueContext, DialogueContextAll, LocalTokenFactory, YarnSpinnerParser,
};
use antlr_rust::char_stream::CharStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::errors::ANTLRError;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::{InputStream, TidAble};
use std::collections::HashMap;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, YarnCompilerError>;

pub fn compile_string(input: &str) -> Result<Program> {
    let input_stream = InputStream::new(input);
    let lexer = YarnSpinnerLexer::new(input_stream);
    let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));
    let dialogue = parser.dialogue()?;
    let program = Program {
        name: "test".to_string(),
        nodes: HashMap::new(),
        initial_values: HashMap::new(),
    };
    Ok(program)
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum YarnCompilerError {
    #[error(transparent)]
    Antlr(#[from] ANTLRError),
}
