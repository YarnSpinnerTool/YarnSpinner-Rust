pub use crate::compiler::compilation_job::*;
use crate::output::*;
use crate::parser::generated::yarnspinnerlexer::YarnSpinnerLexer;
use crate::parser::generated::yarnspinnerparser::YarnSpinnerParser;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;

mod compilation_job;

/// Compile Yarn code, as specified by a compilation job.
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

    CompilationResult {
        program: None,
        string_table: Default::default(),
        declarations: None,
        contains_implicit_string_tags: false,
        file_tags: Default::default(),
        diagnostics: vec![],
        debug_info: Default::default(),
    }
}
