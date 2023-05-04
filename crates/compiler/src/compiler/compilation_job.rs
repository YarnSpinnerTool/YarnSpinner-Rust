//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CompilationJob.cs>

use crate::output::Declaration;
use std::path::Path;
use yarn_slinger_core::prelude::Library;

/// An object that contains Yarn source code to compile, and instructions on
/// how to compile it.
///
/// Instances of this struct are used with `Compiler::compile` to produce
/// `CompilationResult` objects.
#[derive(Debug, Clone, Default, PartialEq)]
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

impl CompilationJob {
    pub fn with_file(mut self, file: File) -> Self {
        self.files.push(file);
        self
    }

    pub fn read_file(mut self, file_path: impl AsRef<Path>) -> std::io::Result<Self> {
        let file_name = file_path.as_ref().to_string_lossy().to_string();
        let file_content = std::fs::read_to_string(file_path)?;
        self.files.push(File {
            file_name,
            source: file_content,
        });
        Ok(self)
    }

    pub fn with_library(mut self, library: Library) -> Self {
        self.library = Some(library);
        self
    }

    pub fn with_compilation_type(mut self, compilation_type: CompilationType) -> Self {
        self.compilation_type = compilation_type;
        self
    }

    pub fn with_variable_declaration(mut self, declaration: Declaration) -> Self {
        self.variable_declarations.push(declaration);
        self
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
