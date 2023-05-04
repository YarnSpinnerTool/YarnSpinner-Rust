pub mod collections;
pub(crate) mod compilation_steps;
pub(crate) mod compiler;
pub(crate) mod error_strategy;
mod file_parse_result;
pub(crate) mod listeners;
mod output;
mod parser;
pub(crate) mod parser_rule_context_ext;
mod string_table_manager;
pub(crate) mod token_ext;
pub(crate) mod visitors;

pub use crate::compiler::Result;

pub mod prelude {
    pub(crate) use crate::{
        compiler::antlr_rust_ext::*, compiler::utils::*, compiler::CompilationIntermediate,
        string_table_manager::*, token_ext::*,
    };
    pub use crate::{
        compiler::compilation_job::*,
        compiler::compile,
        file_parse_result::*,
        listeners::{Diagnostic, DiagnosticSeverity},
        output::*,
        parser::*,
    };
}
