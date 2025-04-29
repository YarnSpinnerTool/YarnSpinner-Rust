//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Analyser.cs>,
//! which was split into multiple files.

use crate::prelude::*;
use core::fmt::{Display, Formatter};
use core::iter;

/// A result of analysing a compiled Yarn program with [`Dialogue::analyse`]. Created by the [`CompiledProgramAnalyser`]s used in the given [`Context`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct Diagnosis {
    /// The severity of the diagnosis. See [`DiagnosisSeverity`] for more information.
    pub severity: DiagnosisSeverity,
    /// The user-friendly message of the diagnosis.
    pub message: String,
    /// The name of the node that caused the diagnosis, if any.
    pub node_name: Option<String>,
    /// The 1-indexed line number of the node that caused the diagnosis, if any.
    pub line: Option<usize>,
    /// The 1-indexed column number, i.e. the character index in the line, of the node that caused the diagnosis, if any.
    pub column: Option<usize>,
}

/// The severity of a [`Diagnosis`], as reported by a [`CompiledProgramAnalyser`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub enum DiagnosisSeverity {
    /// An error.
    ///
    /// Errors represent serious problems that the user should fix before running the Yarn program.
    Error,

    /// A warning.
    ///
    /// Warnings represent possible problems that the user should fix,
    /// but do not cause the compilation process to fail.
    Warning,

    /// A note.
    ///
    /// Notes represent information that the user may find useful, but can safely be ignored.
    Note,
}

impl Diagnosis {
    /// Creates a new [`Diagnosis`] with the given severity and message.
    #[must_use]
    pub fn new(severity: DiagnosisSeverity, message: String) -> Self {
        Self {
            severity,
            message,
            node_name: Default::default(),
            line: Default::default(),
            column: Default::default(),
        }
    }

    /// Sets the node name the diagnosis is associated with. By default, this is `None`.
    #[must_use]
    pub fn with_node_name(mut self, node_name: impl Into<String>) -> Self {
        self.node_name = Some(node_name.into());
        self
    }

    /// Sets the 1-indexed line the diagnosis is associated with. By default, this is `None`.
    #[must_use]
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// Sets the 1-indexed column, i.e. the character index in the line, the diagnosis is associated with. By default, this is `None`.
    #[must_use]
    pub fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }
}

impl Display for Diagnosis {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // Implementation note: The original `showSeverity` flag is treated as always on
        let severity = match self.severity {
            DiagnosisSeverity::Error => "ERROR",
            DiagnosisSeverity::Warning => "WARNING",
            DiagnosisSeverity::Note => "Note",
        }
        .to_owned();
        let line = self.line.map(|line| {
            let column = self.column.map(|c| format!(":{c}")).unwrap_or_default();
            format!("{line}{column}")
        });
        let message = [Some(severity), self.node_name.clone(), line]
            .into_iter()
            .take_while(|o| o.is_some())
            .flatten()
            .chain(iter::once(self.message.clone()))
            .collect::<Vec<_>>()
            .join(": ");
        write!(f, "{message}")
    }
}
