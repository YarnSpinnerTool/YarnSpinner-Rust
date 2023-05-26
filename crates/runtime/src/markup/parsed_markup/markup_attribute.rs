//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>
//! which was split into multiple files.

use crate::markup::{MarkupAttributeMarker, MarkupValue};
#[cfg(any(feature = "bevy", feature = "serde"))]
use crate::prelude::*;
use core::fmt::Display;
use std::collections::HashMap;

/// Represents a range of text in a marked-up string.
///
/// You do not create instances of this struct yourself.
/// It is created by [`Dialogue`] in [`Dialogue::continue_`] and passed to you through a [`DialogueEvent`].
///
/// ## See also
/// - [`Line`]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct MarkupAttribute {
    /// The name of the attribute.
    pub name: String,
    /// The position in the plain text where this attribute begins.
    pub position: usize,
    /// The number of text elements in the plain text that this attribute covers.
    pub length: usize,
    /// The properties associated with this attribute.
    pub properties: HashMap<String, MarkupValue>,
    /// The position of the marker in the original text.
    pub source_position: usize,
}

impl MarkupAttribute {
    pub(crate) fn from_marker(marker: MarkupAttributeMarker, length: usize) -> Self {
        Self {
            name: marker.name.unwrap(),
            position: marker.position,
            length,
            properties: marker.properties,
            source_position: marker.source_position,
        }
    }

    pub fn property(&self, name: &str) -> Option<&MarkupValue> {
        self.properties.get(name)
    }
}

impl Display for MarkupAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties = (!self.properties.is_empty())
            .then(|| format!(", {} properties", self.properties.len()))
            .unwrap_or_default();
        write!(
            f,
            "[{name}] - {start}-{end} ({length}{properties})",
            name = self.name,
            start = self.position,
            end = self.position + self.length,
            length = self.length
        )
    }
}
