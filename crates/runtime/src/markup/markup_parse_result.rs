//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>

pub use self::{markup_attribute::*, markup_value::*};
pub(crate) use self::{markup_attribute_marker::*, tag_type::*};
use std::fmt::Debug;

mod markup_attribute;
mod markup_attribute_marker;
mod markup_value;
mod tag_type;

/// The result of parsing a line of marked-up text.
///
/// You do not create instances of this struct yourself. It is created
/// by objects that can parse markup, such as [`Dialogue`].
///
/// ## Implementation Notes
/// - This is called `MarkupParseResult` in the original C# code, but was renamed because [`Result`] already carries meaning in Rust.
/// - The API has been merged with [`Line`], so this is now only an internal type.

#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub(crate) struct ParsedMarkup {
    /// The original text, with all parsed markers removed.
    pub text: String,
    /// The list of [`MarkupAttribute`] in this parse result.
    pub attributes: Vec<MarkupAttribute>,
}

impl ParsedMarkup {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}
