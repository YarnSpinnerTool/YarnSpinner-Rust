//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Compiler.cs>
//! and <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CompilationJob.cs>

use crate::output::*;
use std::path::Path;
use yarn_slinger_core::prelude::Library;

pub(crate) mod antlr_rust_ext;
pub(crate) mod run_compilation;
pub(crate) mod utils;

pub type Result<T> = std::result::Result<T, CompilationError>;

/// An object that contains Yarn source code to compile, and instructions on
/// how to compile it.
///
/// Consume this information using [`Compiler::compile`] to produce a [`Compilation`] result.
///
/// ## Implementation note
///
/// This type is a combination of the original `CompilationStep` and `Compiler` types, optimized for easier, fluent calling.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Compiler {
    /// The [`File`] structs that represent the content to parse..
    pub files: Vec<File>,

    /// The [`Library`] that contains declarations for functions.
    pub library: Library,

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

    pub fn extend_library(&mut self, library: Library) -> &mut Self {
        self.library.extend(library.into_iter());
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

    pub fn compile(&self) -> Result<Compilation> {
        run_compilation::compile(self)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_call_compile_empty_without_crash() {
        Compiler::new().compile().unwrap();
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
