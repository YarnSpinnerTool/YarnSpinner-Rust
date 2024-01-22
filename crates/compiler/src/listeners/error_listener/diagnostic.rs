use crate::parser_rule_context_ext::ParserRuleContextExt;
use crate::prelude::*;
use annotate_snippets::{Annotation, AnnotationType, Renderer, Slice, Snippet, SourceAnnotation};
use antlr_rust::rule_context::CustomRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_factory::TokenFactory;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use yarn_slinger_core::prelude::*;

/// A diagnostic message that describes an error, warning or informational
/// message that the user can take action on.
///
/// Diagnostics are presented to the user as the result of compilation,
/// through the [`Compilation`]'s [`Compilation::warnings`] field when not an error.
/// Otherwise, they are contained in [`CompilerError`], which is in the [`Err`] variant
/// of the [`Result`] returned by [`Compiler::compile`].
///
/// ## Implementation notes
///
/// The properties marked as `Obsolete` were not implemented.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct Diagnostic {
    /// The path, URI or file-name that the issue occurred in.
    pub file_name: Option<String>,

    /// The range of the file indicated by the [`Diagnostic::file_name`] that the issue occurred in.
    pub range: Option<Range<Position>>,

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
    pub(crate) fn from_message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file_name: Default::default(),
            range: Default::default(),
            context: Default::default(),
            severity: Default::default(),
            start_line: Default::default(),
        }
    }

    pub(crate) fn with_parser_context<'input, T>(
        self,
        ctx: &T,
        token_stream: &ActualTokenStream<'input>,
    ) -> Self
    where
        T: ParserRuleContextExt<'input>,
    <<<<T as CustomRuleContext<'input>>::TF as TokenFactory<'input>>::Inner as Token>::Data as ToOwned>::Owned:
        Into<String>
    {
        let lines_above_and_below_offending_line = 2;
        let lines_around = ctx.get_lines_around(token_stream, lines_above_and_below_offending_line);
        let range = ctx.range();

        self.with_range(range)
            .with_context(lines_around.lines)
            .with_start_line(lines_around.first_line)
    }

    pub(crate) fn with_file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub(crate) fn with_range(mut self, range: impl Into<Range<Position>>) -> Self {
        self.range = Some(range.into());
        self
    }

    pub(crate) fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub(crate) fn with_start_line(mut self, start_line: usize) -> Self {
        self.start_line = start_line;
        self
    }

    pub(crate) fn with_severity(mut self, severity: DiagnosticSeverity) -> Self {
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
                source: self.context.as_deref().unwrap_or("<unknown line>"),
                line_start: self.start_line + 1,
                origin: self.file_name.as_deref(),
                fold: false,
                annotations: vec![SourceAnnotation {
                    label: "",
                    annotation_type,
                    range: convert_absolute_range_to_relative(self),
                }],
            }],
        };
        let renderer = Renderer::styled();
        let annotations = renderer.render(snippet);
        writeln!(f, "{}", annotations)?;

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

    let relative_start_line = range.start.line - diagnostic.start_line;
    let annotated_lines = range.end.line - range.start.line;
    let line_lengths: Vec<_> = context
        .lines()
        .map(|line| line.chars().count() + 1)
        .collect();
    let relative_start =
        line_lengths.iter().take(relative_start_line).sum::<usize>() + range.start.character;
    let relative_end: usize = line_lengths
        .iter()
        .take(relative_start_line + annotated_lines)
        .sum::<usize>()
        + range.end.character
        // - 1 because the Diagnostic range is exclusive, but the annotation range is inclusive
        - 1;
    let mut char_indices = context.char_indices().map(|(i, _)| i);
    let byte_start = char_indices.clone().nth(relative_start).unwrap();
    let byte_end = char_indices.nth(relative_end).unwrap_or(byte_start);
    (byte_start, byte_end)
}

/// Trait implemented for `Vec<Diagnostic>` to provide utility methods.
pub trait DiagnosticVec {
    /// Returns `true` if any of the [`Diagnostic`]s in the vector are of [`DiagnosticSeverity::Error`].
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
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Default, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
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
