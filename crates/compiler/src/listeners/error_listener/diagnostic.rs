use crate::parser_rule_context_ext::ParserRuleContextExt;
use crate::prelude::*;
use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use std::fmt::{Display, Formatter};
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn read_parser_rule_context<'input>(
        mut self,
        ctx: &impl ParserRuleContextExt<'input>,
        token_stream: &ActualTokenStream<'input>,
    ) -> Self {
        let range = ctx.range(token_stream);
        self.range = Some(range);
        self.context = Some(ctx.get_text_with_whitespace(token_stream));
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

impl Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = &self.message;
        let annotation_type = match self.severity {
            DiagnosticSeverity::Error => AnnotationType::Error,
            DiagnosticSeverity::Warning => AnnotationType::Warning,
            DiagnosticSeverity::Info => AnnotationType::Info,
        };
        let snippet = Snippet {
            title: Some(Annotation {
                label: Some(label),
                id: None,
                annotation_type,
            }),
            footer: vec![],
            slices: vec![Slice {
                source: self.context.as_deref().unwrap_or("<unknown line>"),
                line_start: self
                    .range
                    .as_ref()
                    .map(|r| r.start().line)
                    .unwrap_or_default(),
                origin: self.file_name.as_deref(),
                fold: true,
                annotations: vec![SourceAnnotation {
                    label: "",
                    annotation_type,
                    range: (
                        self.range
                            .as_ref()
                            .map(|r| r.start().character)
                            .unwrap_or_default(),
                        self.range
                            .as_ref()
                            .map(|r| r.end().character)
                            .unwrap_or_default(),
                    ),
                }],
            }],
            opt: FormatOptions {
                color: true,
                ..Default::default()
            },
        };

        let display_list = DisplayList::from(snippet);
        writeln!(f, "{}", display_list)?;

        Ok(())
    }
}

pub trait DiagnosticVec {
    fn has_errors(&self) -> bool;
}

impl DiagnosticVec for Vec<Diagnostic> {
    fn has_errors(&self) -> bool {
        self.iter().any(|d| d.severity == DiagnosticSeverity::Error)
    }
}

/// The severity of the issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, strum_macros::Display)]
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
