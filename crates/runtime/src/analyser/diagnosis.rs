//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Analyser.cs>,
//! which was split into multiple files.

#[cfg(any(feature = "bevy", feature = "serde"))]
use crate::prelude::*;
use core::fmt::{Display, Formatter};
use std::iter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct Diagnosis {
    pub severity: DiagnosisSeverity,
    pub message: String,
    pub node_name: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub enum DiagnosisSeverity {
    Error,
    Warning,
    Note,
}

impl Diagnosis {
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

    #[must_use]
    pub fn with_node_name(mut self, node_name: impl Into<String>) -> Self {
        self.node_name = Some(node_name.into());
        self
    }

    #[must_use]
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

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
