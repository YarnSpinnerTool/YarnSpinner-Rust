pub use crate::compiler::compilation_job::*;
use crate::output::*;

mod compilation_job;

/// Compile Yarn code, as specified by a compilation job.
pub fn compile(_compilation_job: CompilationJob) -> CompilationResult {
    // TODO: Implement this :)
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
