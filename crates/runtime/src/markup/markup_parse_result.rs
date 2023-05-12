//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>

pub(crate) use self::markup_attribute_marker::*;
pub use self::{markup_attribute::*, markup_value::*, tag_type::*};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

mod markup_attribute;
mod markup_attribute_marker;
mod markup_value;
mod tag_type;

/// The result of parsing a line of marked-up text.
///
/// You do not create instances of this struct yourself. It is created
///  by objects that can parse markup, such as [`Dialogue`].
#[derive(Default, Debug, Clone)]
#[non_exhaustive]
pub struct MarkupParseResult {
    /// The original text, with all parsed markers removed.
    pub text: String,
    /// The list of <see cref="MarkupAttribute"/>s in this parse result.
    pub attributes: Vec<MarkupAttribute>,
}

impl MarkupParseResult {
    /// Gets the first attribute with the specified name, if present.
    pub fn get_attribute(&self, name: &str) -> Option<&MarkupAttribute> {
        self.attributes.iter().find(|attr| attr.name == name)
    }

    /// Returns the substring of [`text`] covered by the [`attribute`]s Position and Length properties.
    ///
    /// ## Implementation notes:
    /// Instead of returning an empty string if the length is zero, we return none.
    pub fn text_for_attribute(&self, attribute: &MarkupAttribute) -> Option<&str> {
        (attribute.length != 0)
            .then(|| &self.text[attribute.position..attribute.position + attribute.length])
    }
}
