//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ErrorListener.cs>

use crate::output::Position;
use crate::prelude::generated::yarnspinnerparser::YarnSpinnerParserContextType;
use crate::prelude::generated::yarnspinnerparserlistener::YarnSpinnerParserListener;
use crate::prelude::File;
use antlr_rust::char_stream::InputData;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::errors::ANTLRError;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token::Token;
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::tree::ParseTreeListener;
use std::cell::RefCell;
use std::ops::RangeInclusive;
use std::rc::Rc;

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

    pub fn read_parser_rule_context<'a, 'b, 'input>(
        mut self,
        ctx: Rc<impl ParserRuleContext<'input>>,
    ) -> Self {
        let start = Position::from_token(ctx.start());
        let stop = Position::from_token(ctx.stop());
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

pub(crate) struct LexerErrorListener {
    pub(crate) diagnostics: RefCell<Vec<Diagnostic>>,
    file_name: String,
}

impl LexerErrorListener {
    pub(crate) fn new(file_name: String) -> Self {
        Self {
            file_name,
            diagnostics: Default::default(),
        }
    }
}

impl<'input, T: Recognizer<'input>> ErrorListener<'input, T> for LexerErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'input>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&ANTLRError>,
    ) {
        let line = (line - 1) as usize;
        let column = column as usize;
        let range = Position {
            line,
            character: column,
        }..=Position {
            line,
            character: column + 1,
        };
        self.diagnostics.borrow_mut().push(
            Diagnostic::from_message(msg)
                .with_range(range)
                .with_file_name(&self.file_name),
        );
    }
}

pub(crate) struct ParserErrorListener {
    pub(crate) diagnostics: Rc<RefCell<Vec<Diagnostic>>>,
    /// Not in original implementation, but needed because
    /// [`Token::get_source`] is not implemented by antlr4rust.
    /// So we take the entire file instead of just the file name
    /// and extract the lines ourselves.
    file: File,
}

impl ParserErrorListener {
    pub fn new(file: File) -> Self {
        Self {
            diagnostics: Default::default(),
            file,
        }
    }
}

impl<'input, T: Recognizer<'input>> ErrorListener<'input, T> for ParserErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        offending_symbol: Option<&<T::TF as TokenFactory<'input>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&ANTLRError>,
    ) {
        let range = Position {
            line: (line - 1) as usize,
            character: (column + 1) as usize,
        }..=Position {
            line: (line - 1) as usize,
            character: (column + 1) as usize,
        };
        let mut diagnostic = Diagnostic::from_message(msg)
            .with_file_name(&self.file.file_name)
            .with_range(range);
        if let Some(offending_symbol) = offending_symbol {
            let mut string = String::new();

            // the line with the error on it
            let input = &self.file.source;
            let mut lines = input.lines();
            let error_line = lines.nth((line - 1) as usize).unwrap();
            string.push_str(error_line);

            // adding indicator symbols pointing out where the error is
            // on the line
            let start = offending_symbol.get_start();
            let stop = offending_symbol.get_stop();
            if start >= 0 && stop >= 0 {
                // the end point of the error in "line space"
                let end = (stop - start) + column + 1;
                for i in 0..end {
                    // move over until we are at the point we need to be
                    if i >= column && i < end {
                        string.push('^');
                    } else {
                        string.push(' ');
                    }
                }
            }
            let line = (offending_symbol.get_line() - 1) as usize;
            let column = offending_symbol.get_column() as usize;
            let length = offending_symbol.get_text().len();
            diagnostic = diagnostic.with_context(string).with_range(
                Position {
                    line,
                    character: column,
                }..=Position {
                    line,
                    character: column + length,
                },
            );
        }
        self.diagnostics.borrow_mut().push(diagnostic);
    }
}

impl<'input> ParseTreeListener<'input, YarnSpinnerParserContextType> for ParserErrorListener {}
impl<'input> YarnSpinnerParserListener<'input> for ParserErrorListener {}
