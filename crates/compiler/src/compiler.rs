//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Compiler.cs>

use crate::compilation_steps::*;
use crate::listeners::*;
use crate::output::*;
use crate::prelude::*;
use crate::string_table_manager::StringTableManager;
use crate::visitors::*;
pub use compilation_job::*;
use std::collections::{HashMap, HashSet};

pub(crate) mod antlr_rust_ext;
pub mod compilation_job;
pub(crate) mod utils;

pub type Result<T> = std::result::Result<T, CompilationError>;

/// Compile Yarn code, as specified by a compilation job.
pub fn compile(compilation_job: CompilationJob) -> Result<Compilation> {
    let compiler_steps: Vec<&CompilationStep> = vec![
        &register_initial_variables,
        &parse_files,
        &register_strings,
        &validate_unique_node_names,
        &break_on_job_with_only_strings,
        &get_declarations,
        &check_types,
        &find_tracking_nodes,
        &create_declarations_for_tracking_nodes,
        &add_tracking_declarations,
        &resolve_deferred_type_diagnostic,
        &break_on_job_with_only_declarations,
        &generate_code,
        &add_initial_value_registrations,
    ];

    let initial = CompilationIntermediate::from_job(&compilation_job);
    let intermediate = compiler_steps.into_iter().fold(initial, |state, step| {
        if state.early_break {
            state
        } else {
            step(state)
        }
    });
    // Cleaning up diagnostics doesn't change the state but makes sure
    // that diagnostics are unique, there are no errors in the warnings, etc.
    // So we execute it even if we've had early breaks.
    clean_up_diagnostics(intermediate).result.unwrap()
}

type CompilationStep = dyn Fn(CompilationIntermediate) -> CompilationIntermediate;

pub(crate) struct CompilationIntermediate<'input> {
    pub(crate) job: &'input CompilationJob,
    pub(crate) result: Option<Result<Compilation>>,
    /// All variable declarations that we've encountered, PLUS the ones we knew about before
    pub(crate) known_variable_declarations: Vec<Declaration>,
    /// All variable declarations that we've encountered during this compilation job
    pub(crate) derived_variable_declarations: Vec<Declaration>,
    pub(crate) potential_issues: Vec<DeferredTypeDiagnostic>,
    pub(crate) parsed_files: Vec<FileParseResult<'input>>,
    pub(crate) tracking_nodes: HashSet<String>,
    pub(crate) string_table: StringTableManager,
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) file_tags: HashMap<String, Vec<String>>,
    pub(crate) known_types: KnownTypes,
    pub(crate) early_break: bool,
}

impl<'input> CompilationIntermediate<'input> {
    pub(crate) fn from_job(compilation_job: &'input CompilationJob) -> Self {
        Self {
            job: compilation_job,
            result: Default::default(),
            known_variable_declarations: Default::default(),
            derived_variable_declarations: Default::default(),
            potential_issues: Default::default(),
            parsed_files: Default::default(),
            tracking_nodes: Default::default(),
            string_table: Default::default(),
            diagnostics: Default::default(),
            file_tags: Default::default(),
            known_types: Default::default(),
            early_break: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_call_compile_empty_without_crash() {
        compile(CompilationJob::default()).unwrap();
    }

    #[test]
    fn can_call_compile_file_without_crash() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
foo
bar
a {1 + 3} cool expression
==="
            .to_string(),
        };
        compile(CompilationJob::default().with_file(file)).unwrap();
    }
}
