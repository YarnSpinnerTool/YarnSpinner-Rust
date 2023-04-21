//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CompilationResult.cs>

use crate::error_listener::Diagnostic;
pub use crate::output::{debug_info::*, declaration::*, string_info::*};
use crate::prelude::StringTableManager;
use rusty_yarn_spinner_core::prelude::Program;
use std::collections::HashMap;

mod debug_info;
mod declaration;
mod string_info;

/// The result of a compilation.
///
/// Instances of this struct are produced as a result of supplying a [`CompilationJob`] to [`compile`].
#[derive(Default)]
pub struct CompilationResult {
    /// The compiled Yarn program that the [`Compiler`] produced.
    /// produced.
    ///
    /// This value will be [`None`] if there were errors
    /// in the compilation. If this is the case, [`Diagnostics`]
    /// will contain information describing the errors.
    ///
    /// It will also be [`None`] if the <see
    /// [`CompilationJob`] object's [`CompilationJob::CompilationType`] value was not
    /// [`CompilationType::FullCompilation`]
    pub program: Option<Program>,

    /// A dictionary mapping line IDs to StringInfo objects.
    ///
    /// The string table contains the extracted line text found in the
    /// provided source code. The keys of this dictionary are the line IDs
    /// for each line - either through explicit line tags indicated through
    /// the `#line:` tag, or implicitly-generated line IDs that the
    /// compiler added during compilation.
    pub string_table: StringTableManager,

    /// The collection of variable declarations that were found during
    /// compilation.
    ///
    /// This value will be [`None`] if the [`CompilationJob`] object's
    /// [`CompilationType`] value was not [`CompilationType::FullCompilation`].
    pub declarations: Option<Vec<Declaration>>,

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
    /// indicated by the [`CompilationJob.File.FileName`] property
    /// of the [`CompilationJob`]'s [`CompilationJob.Files`] collection), and the values are the
    /// file tags associated with that file.
    pub file_tags: HashMap<String, Vec<String>>,

    /// The collection of [`Diagnostic`] objects that
    /// describe problems in the source code.
    ///
    /// If the compiler encounters errors while compiling source code, the
    /// [`CompilationResult`] it produces will have a [`Program`] value of [`None`]. To help figure out
    /// what the error is, users should consult the contents of this field.
    pub diagnostics: Vec<Diagnostic>,

    /// The collection of [`DebugInfo`] objects for each node
    /// in [`Program`].
    pub debug_info: HashMap<String, DebugInfo>,
}
