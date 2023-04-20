//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ErrorListener.cs>

use crate::output::Position;
use std::ops::RangeInclusive;

/// A diagnostic message that describes an error, warning or informational
/// message that the user can take action on.
///
/// Diagnostics are presented to the user as the result of compilation,
/// through the [`CompilationResult`]'s [`CompilationResult::diagnostics`] field.
///
/// ## Implementation notes
///
/// The properties marked as `Obsolete` were not implemented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// The path, URI or file-name that the issue occurred in.
    pub file_name: String,

    /// The range of the file indicated by the [`Diagnostic::file_name`] that the issue occurred in.
    pub range: RangeInclusive<Position>,

    /// The description of the issue.
    pub message: String,

    /// The source text of [`Diagnostic::file_name`] containing the issue.
    pub context: String,

    /// The severity of the issue.
    pub severity: DiagnosticSeverity,
}

/// The severity of the issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiagnosticSeverity {
    /// An error.
    ///
    /// If a Yarn source file contains errors, it cannot be compiled,
    /// and the compilation process will fail.
    #[default]
    Error,

    /// An warning.
    ///
    /// Warnings represent possible problems that the user should fix,
    /// but do not cause the compilation process to fail.
    Warning,

    /// An informational diagnostic.
    ///
    /// Infos represent possible issues or steps that the user may wish
    /// to fix, but are unlikely to cause problems.
    Info,
}
