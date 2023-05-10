//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Compiler.cs>
//! and <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CompilationJob.cs>
use crate::compilation_steps::*;
use crate::listeners::*;
use crate::output::*;
use crate::prelude::*;
use crate::string_table_manager::StringTableManager;
use crate::visitors::*;
use std::collections::{HashMap, HashSet};

pub(crate) mod antlr_rust_ext;
pub(crate) mod utils;
use crate::output::Declaration;
use std::path::Path;
use yarn_slinger_core::prelude::Library;

pub type Result<T> = std::result::Result<T, CompilationError>;

/// An object that contains Yarn source code to compile, and instructions on
/// how to compile it.
///
/// Consume this information using [`Compiler::compile`] to produce a [`Compilation`] result.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Compiler {
    /// The [`File`] structs that represent the content to parse..
    pub files: Vec<File>,

    /// The [`Library`] that contains declarations for functions.
    pub library: Option<Library>,

    /// The types of compilation that the compiler will do.
    pub compilation_type: CompilationType,

    /// The declarations for variables.
    pub variable_declarations: Vec<Declaration>,
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, file: File) -> &mut Self {
        self.files.push(file);
        self
    }

    pub fn try_read_file(&mut self, file_path: impl AsRef<Path>) -> std::io::Result<&mut Self> {
        let file_name = file_path.as_ref().to_string_lossy().to_string();
        let file_content = std::fs::read_to_string(file_path)?;
        self.files.push(File {
            file_name,
            source: file_content,
        });
        Ok(self)
    }

    pub fn read_file(&mut self, file_path: impl AsRef<Path>) -> &mut Self {
        self.try_read_file(file_path).unwrap()
    }

    pub fn replace_library(&mut self, library: Library) -> &mut Self {
        self.library.replace(library);
        self
    }

    pub fn extend_library(&mut self, mut extend_fn: impl FnMut(&mut Library)) -> &mut Self {
        let library = self.library.get_or_insert_with(Library::default);
        extend_fn(library);
        self
    }

    pub fn compile_until(&mut self, compilation_type: CompilationType) -> &mut Self {
        self.compilation_type = compilation_type;
        self
    }

    pub fn declare_variable(&mut self, declaration: Declaration) -> &mut Self {
        self.variable_declarations.push(declaration);
        self
    }

    pub fn compile(self) -> Result<Compilation> {
        compile(self)
    }
}

/// Represents the contents of a file to compile.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct File {
    /// The name of the file.
    ///
    /// This may be a full path, or just the filename or anything in
    /// between. This is useful for diagnostics, and for attributing
    /// [`Line`] objects to their original source files.
    pub file_name: String,

    /// The source code of this file.
    pub source: String,
}

/// The types of compilation that the compiler will do.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub enum CompilationType {
    /// The compiler will do a full compilation, and generate a [`Program`],
    /// function declaration set, and string table.
    #[default]
    FullCompilation,
    // The compiler will derive only the variable and function declarations,
    // and file tags, found in the script.
    DeclarationsOnly,

    // The compiler will generate a string table only.
    StringsOnly,
}

/// Compile Yarn code, as specified by a compilation job.
fn compile(compilation_job: Compiler) -> Result<Compilation> {
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
    pub(crate) job: &'input Compiler,
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
    pub(crate) fn from_job(compilation_job: &'input Compiler) -> Self {
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
        compile(Compiler::default()).unwrap();
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
        Compiler::new().add_file(file).compile().unwrap();
    }
}
