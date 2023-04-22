//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ErrorListener.cs>

use crate::output::Position;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
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
    pub file_name: Option<String>,

    /// The range of the file indicated by the [`Diagnostic::file_name`] that the issue occurred in.
    pub range: Option<RangeInclusive<Position>>,

    /// The description of the issue.
    pub message: String,

    /// The source text of [`Diagnostic::file_name`] containing the issue.
    pub context: Option<String>,

    /// The severity of the issue.
    pub severity: DiagnosticSeverity,
}

impl Diagnostic {
    pub fn from_message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file_name: Default::default(),
            range: Default::default(),
            context: Default::default(),
            severity: Default::default(),
        }
    }

    pub fn read_parser_rule_context(&mut self, ctx: &impl ParserRuleContext<'_>) -> &mut Self {
        let start = Position::from_token(&ctx.start());
        let stop = Position::from_token(&ctx.stop());
        self.range = Some(start..=stop);
        self.context = Some(ctx.get_text());
        self
    }

    pub fn with_file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub fn with_range(mut self, range: impl Into<RangeInclusive<Position>>) -> Self {
        self.range = Some(range.into());
        self
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn with_severity(mut self, severity: DiagnosticSeverity) -> Self {
        self.severity = severity;
        self
    }
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
