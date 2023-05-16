mod collections;
pub(crate) mod compilation_steps;
pub(crate) mod compiler;
pub(crate) mod error_strategy;
mod feature_gates;
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
        compiler::antlr_rust_ext::*, compiler::run_compilation::*, compiler::utils::*,
        feature_gates::*, file_parse_result::*, parser::*, parser_rule_context_ext::*,
        string_table_manager::*, token_ext::*,
    };
    pub use crate::{
        compiler::{CompilationType, Compiler, File},
        listeners::{Diagnostic, DiagnosticSeverity},
        output::*,
    };
}
