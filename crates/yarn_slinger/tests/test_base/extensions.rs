//! Extension traits that make testing easier

use crate::prelude::*;
use yarn_slinger_compiler::prelude::*;

pub trait CompileResultExt {
    fn unwrap_pretty(self) -> Compilation;
}

impl CompileResultExt for Result<Compilation, CompilationError> {
    fn unwrap_pretty(self) -> Compilation {
        match self {
            Ok(compilation) => compilation,
            Err(error) => {
                for diagnostic in error.diagnostics {
                    eprintln!("{}", diagnostic);
                }
                panic!("Compilation failed due to Yarn errors")
            }
        }
    }
}

pub trait TestCompilationJob {
    fn from_test_source(source: &str) -> Self;
}

impl TestCompilationJob for CompilationJob {
    fn from_test_source(source: &str) -> Self {
        let file = File {
            file_name: "<input>".to_string(),
            source: create_test_node(source),
        };

        Self::default().with_file(file)
    }
}
