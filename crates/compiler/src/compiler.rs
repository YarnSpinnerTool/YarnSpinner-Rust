use crate::compiler::compilation_job::CompilationJob;
use crate::output::*;
use crate::parser::generated::yarnspinnerlexer::YarnSpinnerLexer;
use crate::parser::generated::yarnspinnerparser::YarnSpinnerParser;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;

mod compilation_job;

pub fn compile(_compilation_job: CompilationJob) -> CompilationResult {
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
    let _dialogue = parser
        .dialogue()
        .expect("This error should be handled by the error listener and go into the diagnostics.");
    todo!()
}
