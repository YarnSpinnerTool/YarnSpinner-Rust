use crate::prelude::*;
use bevy::prelude::*;
use yarn_slinger::runtime::{BorrowedLine, CHARACTER_ATTRIBUTE, CHARACTER_ATTRIBUTE_NAME_PROPERTY};

pub(crate) fn localized_line_plugin(_app: &mut App) {}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalizedLine {
    /// The ID of the line in the string table.
    pub id: LineId,
    /// The original text, with all parsed markers removed.
    pub text: String,
    /// The list of [`MarkupAttribute`] in this parse result.
    pub attributes: Vec<MarkupAttribute>,
    pub asset: Option<HandleUntyped>,
}
impl LocalizedLine {
    // Documentation taken from `YarnLine`
    /// Gets the first attribute with the specified name, if present.
    pub fn attribute(&self, name: &str) -> Option<&MarkupAttribute> {
        self.borrow().attribute(name)
    }

    // Documentation taken from `YarnLine`
    /// The name of the character, if present.
    /// ## Examples
    /// When there is a name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bevy_yarn_slinger::prelude::*;
    /// # let line = LocalizedLine {
    /// #    id: "line".into(),
    /// #    text: "Alice: Hello! How are you today?".to_owned(),
    /// #    attributes: vec![MarkupAttribute {
    /// #        name: "character".to_owned(),
    /// #        position: 0,
    /// #        length: 7,
    /// #        properties: HashMap::from([("name".to_owned(), "Alice".into())]),
    /// #        source_position: 0,
    /// #    }],
    /// #    asset: None,
    /// # };
    /// assert_eq!("Alice: Hello! How are you today?", line.text);
    /// assert_eq!(Some("Alice"), line.character_name());
    /// ```
    ///
    /// When there is no name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bevy_yarn_slinger::prelude::*;
    /// # let line = LocalizedLine {
    /// #    id: "line".into(),
    /// #    text: "Great, thanks".to_owned(),
    /// #    attributes: vec![],
    /// #    asset: None,
    /// # };
    /// assert_eq!("Great, thanks", line.text);
    /// assert!(line.character_name().is_none());
    pub fn character_name(&self) -> Option<&str> {
        self.borrow().character_name()
    }

    // Documentation taken from `YarnLine`
    /// The underlying text for this line, with any `character` attribute removed.
    ///
    /// ## Examples
    /// When there is a name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bevy_yarn_slinger::prelude::*;
    /// # let line = LocalizedLine {
    /// #    id: "line".into(),
    /// #    text: "Alice: Hello! How are you today?".to_owned(),
    /// #    attributes: vec![MarkupAttribute {
    /// #        name: "character".to_owned(),
    /// #        position: 0,
    /// #        length: 7,
    /// #        properties: HashMap::from([("name".to_owned(), "Alice".into())]),
    /// #        source_position: 0,
    /// #    }],
    /// #    asset: None,
    /// # };
    /// assert_eq!("Alice: Hello! How are you today?", line.text);
    /// assert_eq!("Hello! How are you today?", &line.text_without_character_name());
    /// ```
    ///
    /// When there is no name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bevy_yarn_slinger::prelude::*;
    /// # let line = LocalizedLine {
    /// #    id: "line".into(),
    /// #    text: "Great, thanks".to_owned(),
    /// #    attributes: vec![],
    /// #    asset: None,
    /// # };
    /// assert_eq!("Great, thanks", line.text);
    /// assert_eq!("Great, thanks", &line.text_without_character_name());
    pub fn text_without_character_name(&self) -> String {
        self.borrow().text_without_character_name()
    }

    // Documentation taken from `YarnLine`
    /// Returns the substring of [`Line::text`] covered by the passed `attribute`s [`MarkupAttribute::position`] and [`MarkupAttribute::length`] fields.
    pub fn text_for_attribute(&self, attribute: &MarkupAttribute) -> &str {
        self.borrow().text_for_attribute(attribute)
    }

    // Documentation taken from `YarnLine`
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
    /// This method does not modify the current object. A new  [`LocalizedLine`] is returned.
    ///
    /// ## Panics
    /// Panics if `attribute_to_delete` is not an attribute of this [`Line::attribute`].
    pub fn delete_range(&self, attribute_to_delete: &MarkupAttribute) -> Self {
        let line = self.borrow().delete_range(attribute_to_delete);
        Self::from_yarn_line(line, self.asset.clone())
    }

    pub(crate) fn borrow(&self) -> BorrowedYarnLine {
        BorrowedYarnLine {
            id: &self.id,
            text: &self.text,
            attributes: &self.attributes,
        }
    }
}

impl From<LocalizedLine> for YarnLine {
    fn from(line: LocalizedLine) -> Self {
        Self {
            id: line.id,
            text: line.text,
            attributes: line.attributes,
        }
    }
}

impl<'a> From<&'a LocalizedLine> for BorrowedLine<'a> {
    fn from(line: &'a LocalizedLine) -> Self {
        line.borrow()
    }
}

impl LocalizedLine {
    pub(crate) fn from_yarn_line(line: YarnLine, asset: impl Into<Option<HandleUntyped>>) -> Self {
        Self {
            id: line.id,
            text: line.text,
            attributes: line.attributes,
            asset: asset.into(),
        }
    }
}
