//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CompilationResult.cs>

use crate::listeners::*;
pub use crate::output::{debug_info::*, declaration::*, string_info::*};
use crate::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};
use yarnspinner_core::prelude::*;

mod debug_info;
mod declaration;
mod string_info;

/// The result of a compilation.
///
/// Instances of this struct are produced as a result of calling [`Compiler::compile`].
///
/// ## Implementation Notes
///
/// In contrast to the original implementation, where this struct was called a `CompilationResult`, we return
/// an actual [`Result`], so this type is guaranteed to only hold warnings as opposed to all diagnostics.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct Compilation {
    /// The compiled Yarn program that the [`Compiler`] produced.
    /// produced.
    ///
    /// This value will be [`None`] if the
    /// [`Compiler`] object's [`Compiler::compilation_type`] value was not
    /// [`CompilationType::FullCompilation`]
    pub program: Option<Program>,

    /// A dictionary mapping line IDs to StringInfo objects.
    ///
    /// The string table contains the extracted line text found in the
    /// provided source code. The keys of this dictionary are the line IDs
    /// for each line - either through explicit line tags indicated through
    /// the `#line:` tag, or implicitly-generated line IDs that the
    /// compiler added during compilation.
    pub string_table: HashMap<LineId, StringInfo>,

    /// The collection of variable declarations that were found during
    /// compilation.
    ///
    /// This value will be empty if the [`Compiler`] object's
    /// [`CompilationType`] value was not [`CompilationType::FullCompilation`].
    pub declarations: Vec<Declaration>,

    /// A value indicating whether the compiler had to create line IDs
    /// for lines in the source code that lacked `#line:` tags.
    ///
    /// Every line is required to have a line ID. If a line doesn't have a
    /// line ID specified in the source code (via a `#line:` tag), the
    /// compiler will create one.
    ///
    /// Implicit line IDs are guaranteed to remain the same between
    /// compilations when the source file does not change. If you want line
    /// IDs to remain the same when the source code may be modified in the
    /// future, add a `#line:` tag to the line. This may be done by
    /// hand, or added using the [`Utility.AddTagsToLines`] method.
    pub contains_implicit_string_tags: bool,

    /// The collection of file-level tags found in the source code.
    ///
    /// The keys of this dictionary are the file names (as
    /// indicated by the [`File::file_name`] field
    /// of the [`Compiler`]'s [`Compiler::files`] collection), and the values are the
    /// file tags associated with that file.
    pub file_tags: HashMap<String, Vec<String>>,

    /// The collection of [`Diagnostic`] objects that
    /// describe possible problems that the user should fix,
    /// but do not cause the compilation process to fail.
    ///
    /// All diagnostics in this collection have a severity of [`DiagnosticSeverity::Warning`].
    /// If there was an error during compilation, the compilation returns an [`Err`] variant containing
    /// error diagnostics instead of this [`Compilation`].
    pub warnings: Vec<Diagnostic>,

    /// The collection of [`DebugInfo`] objects for each node in [`Program`].
    pub debug_info: HashMap<String, DebugInfo>,
}

impl Compilation {
    /// Combines multiple [`CompilationResult`] objects together into one object.
    pub(crate) fn combine(
        compilations: impl Iterator<Item = Compilation>,
        string_table_manager: StringTableManager,
    ) -> Self {
        let mut programs = Vec::new();
        let mut declarations = Vec::new();
        let mut tags = HashMap::new();
        let mut diagnostics = Vec::new();
        let mut node_debug_infos = HashMap::new();

        for compilation in compilations {
            programs.push(compilation.program.unwrap());
            declarations.extend(compilation.declarations);
            tags.extend(compilation.file_tags);
            diagnostics.extend(compilation.warnings);
            node_debug_infos.extend(compilation.debug_info);
        }
        let combined_program = Program::combine(programs);
        let contains_implicit_string_tags = string_table_manager.contains_implicit_string_tags();
        Compilation {
            program: combined_program,
            string_table: string_table_manager.0,
            declarations,
            debug_info: node_debug_infos,
            contains_implicit_string_tags,
            file_tags: tags,
            warnings: diagnostics,
        }
    }
}

/// A collection of [`Diagnostic`] objects that describe problems that occurred during compilation.
/// At least one of these diagnostics will have a severity of [`DiagnosticSeverity::Error`].
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct CompilerError(pub Vec<Diagnostic>);

impl Error for CompilerError {}

impl Debug for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This looks evil, but we support really nice error messages through Display,
        // which we want to print by default on `unwrap` and `expect`.
        // If you really need the real debug print, call [`CompilationError::debug_fmt`].
        Display::fmt(self, f)
    }
}

impl CompilerError {
    /// Acts as a [`Debug`] implementation that does not pretty-print the diagnostics.
    pub fn debug_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(&self.0).finish()
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for diagnostic in &self.0 {
            writeln!(f, "{}", diagnostic)?;
        }
        Ok(())
    }
}
