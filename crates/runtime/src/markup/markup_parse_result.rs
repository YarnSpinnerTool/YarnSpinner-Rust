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
    /// The list of [`MarkupAttribute`] in this parse result.
    pub attributes: Vec<MarkupAttribute>,
}

impl MarkupParseResult {
    /// Gets the first attribute with the specified name, if present.
    pub fn get_attribute(&self, name: &str) -> Option<&MarkupAttribute> {
        self.attributes.iter().find(|attr| attr.name == name)
    }

    /// Returns the substring of [`text`] covered by the [`attribute`]s position and length fields.
    pub fn text_for_attribute(&self, attribute: &MarkupAttribute) -> &str {
        assert!(self.text.len() < attribute.position + attribute.length, "Attribute represents a range not representable by this text. Does this MarkupAttribute belong to this MarkupParseResult?");
        &self.text[attribute.position..attribute.position + attribute.length]
    }

    /// Deletes an attribute from this markup.
    /// This method deletes the range of text covered by `attribute_to_delete`,
    /// and updates the other attributes in this markup as follows:
    ///
    /// - Attributes that start and end before the deleted attribute are
    /// unmodified.
    /// - Attributes that start before the deleted attribute and end inside it
    /// are truncated to remove the part overlapping the deleted attribute.
    /// - Attributes that have the same position and length as the deleted
    /// attribute are deleted, if they apply to any text.
    /// - Attributes that start and end within the deleted attribute are deleted.
    /// - Attributes that start within the deleted attribute, and end outside
    /// it, have their start truncated to remove the part overlapping the
    /// deleted attribute.
    /// - Attributes that start after the deleted attribute have their start
    /// point adjusted to account for the deleted text.
    ///
    /// This method does not modify the current object. A new <see
    /// [`MarkupParseResult`] is returned.
    ///
    /// If `attribute_to_delete` is not an attribute of this
    /// [`MarkupParseResult`], the behaviour is undefined.
    pub fn delete_range(&self, attribute_to_delete: MarkupAttribute) -> MarkupParseResult {
        // Address the trivial case: if the attribute has a zero
        // length, just create a new markup that doesn't include it.
        // The plain text is left unmodified, because this attribute
        // didn't apply to any text.
        if attribute_to_delete.length == 0 {
            let attributes = self
                .attributes
                .iter()
                .filter(|attr| **attr != attribute_to_delete)
                .cloned()
                .collect();
            return MarkupParseResult {
                text: self.text.clone(),
                attributes,
            };
        }
        todo!()
    }
}
