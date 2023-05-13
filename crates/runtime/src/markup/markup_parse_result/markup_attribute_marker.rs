//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>
//! which was split into multiple files.

use crate::markup::{MarkupValue, TagType};
use std::collections::HashMap;

/// Represents a marker (e.g. `[a]`) in line of marked up text.
///
/// You do not create instances of this struct yourself. It is created
/// by objects that can parse markup, such as [`Dialogue`].
pub(crate) struct MarkupAttributeMarker {
    /// The name of the marker.
    /// For example, the marker `[wave]` has the name `wave`.
    pub(crate) name: Option<String>,
    /// The position of the marker in the plain text.
    pub(crate) position: usize,
    /// The list of properties associated with this marker.
    pub(crate) properties: HashMap<String, MarkupValue>,
    /// The type of marker that this is.
    pub(crate) tag_type: TagType,
    /// The position of this marker in the original source text.
    pub(crate) source_position: usize,
}
