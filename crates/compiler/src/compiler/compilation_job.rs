//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CompilationJob.cs>

use crate::output::Declaration;
use yarn_slinger_core::prelude::Library;

/// An object that contains Yarn source code to compile, and instructions on
/// how to compile it.
///
/// Instances of this struct are used with `Compiler::compile` to produce
/// `CompilationResult` objects.
#[derive(Debug, Clone, Default)]
pub struct CompilationJob {
    /// The [`File`] structs that represent the content to parse..
    pub files: Vec<File>,

    /// The [`Library`] that contains declarations for functions.
    pub library: Option<Library>,

    /// The types of compilation that the compiler will do.
    pub compilation_type: CompilationType,

    /// The declarations for variables.
    pub variable_declarations: Vec<Declaration>,
}

/// Represents the contents of a file to compile.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Default)]
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
