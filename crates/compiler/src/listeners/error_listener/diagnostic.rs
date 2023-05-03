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

    /// The line the context starts on.
    pub start_line: usize,
}

impl Diagnostic {
    pub fn from_message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file_name: Default::default(),
            range: Default::default(),
            context: Default::default(),
            severity: Default::default(),
            start_line: Default::default(),
        }
    }

    pub(crate) fn with_parser_context<'input>(
        self,
        ctx: &impl ParserRuleContextExt<'input>,
        token_stream: &ActualTokenStream<'input>,
    ) -> Self {
        let lines_above_and_below_offending_line = 2;
        let lines_around = ctx.get_lines_around(token_stream, lines_above_and_below_offending_line);
        let range = ctx.range(token_stream);
        self.with_range(range)
            .with_context(lines_around.lines)
            .with_start_line(lines_around.first_line)
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

    pub fn with_start_line(mut self, start_line: usize) -> Self {
        self.start_line = start_line;
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
        };
        let snippet = Snippet {
            title: Some(Annotation {
                label: Some(label),
                id: None,
                annotation_type,
            }),
            footer: vec![],
            slices: vec![Slice {
                source: dbg!(self.context.as_deref().unwrap_or("<unknown line>")),
                line_start: dbg!(self.start_line + 1),
                origin: self.file_name.as_deref(),
                fold: false,
                annotations: vec![SourceAnnotation {
                    label: "",
                    annotation_type,
                    range: convert_absolute_range_to_relative(self),
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

fn convert_absolute_range_to_relative(diagnostic: &Diagnostic) -> (usize, usize) {
    let Some(range) = diagnostic.range.as_ref() else {
        return (0, 0);
    };
    let Some(context) = diagnostic.context.as_ref() else {
        return (0, 0);
    };

    let relative_start_line = range.start().line - diagnostic.start_line;
    let annotated_lines = range.end().line - range.start().line;
    let line_lengths: Vec<_> = context.lines().map(|line| line.len() + 1).collect();
    let relative_start =
        line_lengths.iter().take(relative_start_line).sum::<usize>() + range.start().character;
    let relative_end: usize = line_lengths
        .iter()
        .take(relative_start_line + annotated_lines)
        .sum::<usize>()
        + range.end().character;

    (relative_start, relative_end)
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
///
/// ## Implementation notes
///
/// The `Info` variant was not implemented because it was unused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, strum_macros::Display)]
pub enum DiagnosticSeverity {
    /// An error.
    ///
    /// If a Yarn source file contains errors, it cannot be compiled,
    /// and the compilation process will fail.
    #[default]
    Error,

    /// A warning.
    ///
    /// Warnings represent possible problems that the user should fix,
    /// but do not cause the compilation process to fail.
    Warning,
}
