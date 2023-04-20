//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ErrorListener.cs>

/*
    /// <summary>
       /// Gets or sets the path, URI or file-name that the issue occurred in.
       /// </summary>
       public string FileName { get; set; } = "(not set)";

       /// <summary>
       /// Gets or sets the range of the file indicated by <see
       /// cref="FileName"/> that the issue occurred in.
       /// </summary>
       public Range Range { get; set; } = new Range();

       /// <summary>
       /// Gets or sets the description of the issue.
       /// </summary>
       public string Message { get; set; } = "(internal error: no message provided)";

       /// <summary>
       /// Gets or sets the source text of <see cref="FileName"/> containing
       /// the issue.
       /// </summary>
       public string Context { get; set; } = null;

       /// <summary>
       /// Gets or sets the severity of the issue.
       /// </summary>
       public DiagnosticSeverity Severity { get; set; } = DiagnosticSeverity.Error;

       /// <summary>
       /// Gets the zero-indexed line number in FileName at which the issue
       /// begins.
       /// </summary>
       [Obsolete("Use Range.Start.Line")]
       public int Line => Range.Start.Line;

       /// <summary>
       /// Gets the zero-indexed character number in FileName at which the
       /// issue begins.
       /// </summary>
       [Obsolete("Use Range.Start.Character")]
       public int Column => Range.Start.Character;

       /// <summary>
       /// Initializes a new instance of the <see cref="Diagnostic"/> class.
       /// </summary>
       /// <param name="fileName"><inheritdoc cref="FileName"
       /// path="/summary/node()"/></param>
       /// <param name="message"><inheritdoc cref="Message"
       /// path="/summary/node()"/></param>
       /// <param name="severity"><inheritdoc cref="Severity"
       /// path="/summary/node()"/></param>
       public Diagnostic(string fileName, string message, DiagnosticSeverity severity = DiagnosticSeverity.Error)
       {
           this.FileName = fileName;
           this.Message = message;
           this.Severity = severity;
       }

       /// <summary>
       /// Initializes a new instance of the <see cref="Diagnostic"/> class.
       /// </summary>
       /// <param name="message"><inheritdoc cref="Message"
       /// path="/summary/node()"/></param>
       /// <param name="severity"><inheritdoc cref="Severity"
       /// path="/summary/node()"/></param>
       public Diagnostic(string message, DiagnosticSeverity severity = DiagnosticSeverity.Error)
       : this(null, message, severity)
       {
       }

       /// <summary>
       /// Initializes a new instance of the <see cref="Diagnostic"/> class.
       /// </summary>
       /// <param name="fileName"><inheritdoc cref="FileName"
       /// path="/summary/node()"/></param>
       /// <param name="context">The parse node at which the error
       /// occurred.</param>
       /// <param name="message"><inheritdoc cref="Message"
       /// path="/summary/node()"/></param>
       /// <param name="severity"><inheritdoc cref="Severity"
       /// path="/summary/node()"/></param>
       public Diagnostic(string fileName, ParserRuleContext context, string message, DiagnosticSeverity severity = DiagnosticSeverity.Error)
       {
           this.FileName = fileName;

           if (context != null)
           {
               this.Range = new Range(
                   context.Start.Line - 1,
                   context.Start.Column,
                   context.Stop.Line - 1,
                   context.Stop.Column + context.Stop.Text.Length);
           }
           this.Message = message;
           this.Context = context.GetTextWithWhitespace();
           this.Severity = severity;
       }

       /// <summary>
       /// Initializes a new instance of the <see cref="Diagnostic"/> class.
       /// </summary>
       /// <param name="fileName"><inheritdoc cref="FileName"
       /// path="/summary/node()"/></param>
       /// <param name="range"><inheritdoc cref="Range"
       /// path="/summary/node()"/></param>
       /// <param name="message"><inheritdoc cref="Message"
       /// path="/summary/node()"/></param>
       /// <param name="severity"><inheritdoc cref="Severity"
       /// path="/summary/node()"/></param>
       public Diagnostic(string fileName, Range range, string message, DiagnosticSeverity severity = DiagnosticSeverity.Error) {
           this.FileName = fileName;
           this.Range = range;
           this.Message = message;
           this.Severity = severity;
       }

       /// <summary>
       /// The severity of the issue.
       /// </summary>
       public enum DiagnosticSeverity
       {
           /// <summary>
           /// An error.
           /// </summary>
           /// <remarks>
           /// If a Yarn source file contains errors, it cannot be compiled,
           /// and the compilation process will fail.
           /// </remarks>
           Error,

           /// <summary>
           /// An warning.
           /// </summary>
           /// <remarks>
           /// Warnings represent possible problems that the user should fix,
           /// but do not cause the compilation process to fail.
           /// </remarks>
           Warning,

           /// <summary>
           /// An informational diagnostic.
           /// </summary>
           /// <remarks>
           /// Infos represent possible issues or steps that the user may wish
           /// to fix, but are unlikely to cause problems.
           /// </remarks>
           Info,
       }
*/

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
