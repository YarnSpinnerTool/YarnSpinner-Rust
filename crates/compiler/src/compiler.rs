use crate::compiler::compilation_job::CompilationJob;
use crate::output::*;
use crate::parser::generated::yarnspinnerlexer::YarnSpinnerLexer;
use crate::parser::generated::yarnspinnerparser::YarnSpinnerParser;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::errors::ANTLRError;
use antlr_rust::InputStream;
use thiserror::Error;

mod compilation_job;

pub type Result<T> = std::result::Result<T, CompilationError>;

pub fn compile(compilation_job: CompilationJob) -> Result<CompilationOutput> {
    let lexer = YarnSpinnerLexer::new(InputStream::new(
        "# hello
# nonono
title: Node_Title
---
Here are some lines!
That's weird?
Wow!
==="
        .into(),
    ));
    let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));
    let dialogue = parser.dialogue()?;
    todo!()
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CompilationError {
    #[error(transparent)]
    Antlr(#[from] ANTLRError),
    #[error("TODO")]
    Diagnostics,
}
