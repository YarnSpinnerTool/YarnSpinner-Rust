//! Extension traits that make testing easier

use crate::prelude::*;
use yarnspinner_compiler::prelude::*;

pub trait TestCompiler {
    fn from_test_source(source: &str) -> Self;
}

impl TestCompiler for Compiler {
    fn from_test_source(source: &str) -> Self {
        let file = File {
            file_name: "<input>".to_string(),
            source: create_test_node(source),
        };

        let mut compiler = Self::new();
        compiler.add_file(file);
        compiler
    }
}
