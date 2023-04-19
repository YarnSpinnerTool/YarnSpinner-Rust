//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CompilationResult.cs>
//! Renamed to `CompilationOutput` because the term [`Result`] means something else in Rust.

pub use crate::output::{debug_info::*, declaration::*, string_info::*};
use rusty_yarn_spinner_core::prelude::Program;
use std::collections::HashMap;

mod debug_info;
mod declaration;
mod string_info;

/// The result of a compilation.
///
/// Instances of this struct are produced as a result of supplying a [`CompilationJob`] to [`compile`].
///
/// ## Implementation notes
/// Currently only implements `CompilationJob.Type.FullCompilation`, so many fields are not `Option`s.
/// Diagnostics are implemented as the `Err` variant of `Result`.
pub struct CompilationOutput {
    /// The compiled Yarn program that the [`Compiler`]
    /// produced.
    pub program: Program,

    /// Gets a dictionary mapping line IDs to StringInfo objects.
    ///
    /// The string table contains the extracted line text found in the
    /// provided source code. The keys of this dictionary are the line IDs
    /// for each line - either through explicit line tags indicated through
    /// the `#line:` tag, or implicitly-generated line IDs that the
    /// compiler added during compilation.
    pub string_table: HashMap<String, StringInfo>,

    /// Gets the collection of variable declarations that were found during
    /// compilation.
    pub declarations: Vec<Declaration>,

    /// Gets a value indicating whether the compiler had to create line IDs
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

    /// Gets the collection of file-level tags found in the source code.
    ///
    /// The keys of this dictionary are the file names (as
    /// indicated by the [`CompilationJob.File.FileName`] property
    /// of the [`CompilationJob`]'s [`CompilationJob.Files`] collection), and the values are the
    /// file tags associated with that file.
    pub file_tags: HashMap<String, Vec<String>>,

    /// Gets the collection of [`DebugInfo`] objects for each node
    /// in [`Program`].
    pub debug_info: HashMap<String, DebugInfo>,
}
