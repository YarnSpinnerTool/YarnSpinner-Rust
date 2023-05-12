//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>
//! which was split into multiple files.

use crate::markup::MarkupValue;
use std::collections::HashMap;

/// Represents a range of text in a marked-up string.
///
/// You do not create instances of this struct yourself.
/// It is created by objects that can parse markup, such as [`Dialogue`].
///
/// ## See also
/// - [`Dialogue::parse_markup`]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
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
