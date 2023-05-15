//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
//!
//! ## Implementation notes
//! Introduced `LineId` newtype for better type safety

use crate::markup::{
    MarkupAttribute, MarkupValue, CHARACTER_ATTRIBUTE, CHARACTER_ATTRIBUTE_NAME_PROPERTY,
};
use yarn_slinger_core::prelude::*;

/// A line of dialogue, sent from the [`Dialogue`] to the game.
///
/// When the game receives a [`Line`], it should do the following things to prepare the line for presentation to the user.
/// - Use the value in the [`Line::ID`] field to look up the appropriate user-facing text in the string table.
/// - Use [`Dialogue::expand_substitutions`] to replace all substitutions in the user-facing text.
/// - Use [`Dialogue::parse_markup`] to parse all markup in the line.
///
/// You do not create instances of this struct yourself. They are created by the [`Dialogue`] during program execution.
///
/// ## See also
/// [`Dialogue::line_handler`]
///
/// ## Implementation Notes
///
/// `MarkupParseResult` was merged into this because we don't require consumers to manually fetch from string tables.
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    /// The ID of the line in the string table.
    pub id: LineId,
    /// The original text, with all parsed markers removed.
    pub text: String,
    /// The list of [`MarkupAttribute`] in this parse result.
    pub attributes: Vec<MarkupAttribute>,
}

impl Line {
    /// Gets the first attribute with the specified name, if present.
    ///
    /// ## Implementation note
    ///
    /// Originally named `TryGetAttributeWithName`
    pub fn attribute(&self, name: &str) -> Option<&MarkupAttribute> {
        self.attributes.iter().find(|attr| attr.name == name)
    }

    /// The name of the character, if present.
    /// ## Examples
    /// When there is a name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use yarn_slinger_core::prelude::*;
    /// # use yarn_slinger_runtime::markup::*;
    /// # use yarn_slinger_runtime::prelude::*;
    /// # let line = Line {
    /// #    id: "line".into(),
    /// #    text: "Alice: Hello! How are you today?".to_owned(),
    /// #    attributes: vec![MarkupAttribute {
    /// #        name: "character".to_owned(),
    /// #        position: 0,
    /// #        length: 7,
    /// #        properties: HashMap::from([("name".to_owned(), "Alice".into())]),
    /// #        source_position: 0,
    /// #    }]
    /// # };
    /// assert_eq!("Alice: Hello! How are you today?", line.text);
    /// assert_eq!(Some("Alice"), line.character_name());
    /// ```
    ///
    /// When there is no name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use yarn_slinger_core::prelude::*;
    /// # use yarn_slinger_runtime::markup::*;
    /// # use yarn_slinger_runtime::prelude::*;
    /// # let line = Line {
    /// #    id: "line".into(),
    /// #    text: "Great, thanks".to_owned(),
    /// #    attributes: vec![]
    /// # };
    /// assert_eq!("Great, thanks", line.text);
    /// assert!(line.character_name().is_none());
    pub fn character_name(&self) -> Option<&str> {
        if let Some(attribute) = self.attribute(CHARACTER_ATTRIBUTE) {
            if let Some(name) = attribute.property(CHARACTER_ATTRIBUTE_NAME_PROPERTY) {
                let MarkupValue::String(name) = name else {
                    panic!(
                        "Attribute \"character\" has a \"name\" property, but it is not a string. \
                         This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new"
                    );
                };
                return Some(name.as_str());
            }
        }
        None
    }

    /// The underlying text for this line, with any `character` attribute removed.
    ///
    /// ## Examples
    /// When there is a name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use yarn_slinger_core::prelude::*;
    /// # use yarn_slinger_runtime::markup::*;
    /// # use yarn_slinger_runtime::prelude::*;
    /// # let line = Line {
    /// #    id: "line".into(),
    /// #    text: "Alice: Hello! How are you today?".to_owned(),
    /// #    attributes: vec![MarkupAttribute {
    /// #        name: "character".to_owned(),
    /// #        position: 0,
    /// #        length: 7,
    /// #        properties: HashMap::from([("name".to_owned(), "Alice".into())]),
    /// #        source_position: 0,
    /// #    }]
    /// # };
    /// assert_eq!("Alice: Hello! How are you today?", line.text);
    /// assert_eq!("Hello! How are you today?", &line.text_without_character_name());
    /// ```
    ///
    /// When there is no name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use yarn_slinger_core::prelude::*;
    /// # use yarn_slinger_runtime::markup::*;
    /// # use yarn_slinger_runtime::prelude::*;
    /// # let line = Line {
    /// #    id: "line".into(),
    /// #    text: "Great, thanks".to_owned(),
    /// #    attributes: vec![]
    /// # };
    /// assert_eq!("Great, thanks", line.text);
    /// assert_eq!("Great, thanks", &line.text_without_character_name());
    pub fn text_without_character_name(&self) -> String {
        if let Some(attribute) = self.attribute(CHARACTER_ATTRIBUTE) {
            self.delete_range(attribute).text
        } else {
            self.text.to_owned()
        }
    }

    /// Returns the substring of [`text`] covered by the [`attribute`]s position and length fields.
    pub fn text_for_attribute(&self, attribute: &MarkupAttribute) -> &str {
        assert!(
            self.text.len() <= attribute.position + attribute.length,
            "Attribute \"{attribute}\" represents a range not representable by this text: \"{}\". \
        Does this MarkupAttribute belong to this MarkupParseResult?",
            self.text
        );
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
    /// [`ParsedMarkup`] is returned.
    ///
    /// If `attribute_to_delete` is not an attribute of this
    /// [`ParsedMarkup`], the behaviour is undefined.
    pub fn delete_range(&self, attribute_to_delete: &MarkupAttribute) -> Self {
        // Address the trivial case: if the attribute has a zero
        // length, just create a new markup that doesn't include it.
        // The plain text is left unmodified, because this attribute
        // didn't apply to any text.
        if attribute_to_delete.length == 0 {
            let attributes = self
                .attributes
                .iter()
                .filter(|attr| *attr != attribute_to_delete)
                .cloned()
                .collect();
            Self {
                id: self.id.clone(),
                text: self.text.clone(),
                attributes,
            };
        }
        let deletion_start = attribute_to_delete.position;
        let deletion_end = attribute_to_delete.position + attribute_to_delete.length;
        let edited_substring = {
            let mut text = self.text.clone();
            text.replace_range(deletion_start..deletion_end, "");
            text
        };
        let attributes = self
            .attributes
            .iter()
            // This is the attribute we're deleting. Don't include it.
            .filter(|attr| *attr != attribute_to_delete)
            .filter_map(|attribute| {
                let mut attribute = attribute.clone();
                let start = attribute.position;
                let end = attribute.position + attribute.length;
                if start <= deletion_start {
                    // The attribute starts before start point of the item
                    // we're deleting.
                    if end <= deletion_start {
                        // This attribute is entirely before the item we're
                        // deleting, and will be unmodified.
                    } else if end <= deletion_end {
                        // This attribute starts before the item we're
                        // deleting, and ends inside it. The Position
                        // doesn't need to change, but its Length is
                        // trimmed so that it ends where the deleted
                        // attribute begins.
                        let original_length = attribute.length;
                        attribute.length = deletion_start.saturating_sub(start);
                        if original_length > 0 && attribute.length == 0 {
                            // The attribute's length has been reduced to
                            // zero. All of the contents it previous had
                            // have been removed, so we will remove the
                            // attribute itself.
                            return None;
                        }
                    } else {
                        // This attribute starts before the item we're
                        // deleting, and ends after it. Its length is
                        // edited to remove the length of the item we're
                        // deleting.
                        attribute.length =
                            attribute.length.saturating_sub(attribute_to_delete.length);
                    }
                } else if start >= deletion_end {
                    // The item begins after the item we're deleting. Its
                    // length isn't changing. We just need to offset its
                    // start position.
                    attribute.position = start.saturating_sub(attribute_to_delete.length);
                } else if start >= deletion_start && end <= deletion_end {
                    // The item is entirely within the item we're deleting.
                    // It will be deleted too - we'll skip including it in
                    // the updated attributes list.
                    return None;
                } else if start >= deletion_start && end > deletion_end {
                    // The item starts within the item we're deleting, and
                    // ends outside it. We'll adjust the start point so
                    // that it begins at the point where this item and the
                    // item we're deleting stop overlapping.
                    let overlap = deletion_end - start;
                    let new_start = deletion_start;
                    let new_length = attribute.length - overlap;
                    attribute.position = new_start;
                    attribute.length = new_length;
                }
                Some(attribute)
            })
            .collect();
        Self {
            id: self.id.clone(),
            text: edited_substring,
            attributes,
        }
    }
}
