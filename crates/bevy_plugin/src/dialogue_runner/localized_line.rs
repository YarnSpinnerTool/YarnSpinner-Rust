use crate::line_provider::LineAssets;
use crate::prelude::*;
use bevy::prelude::*;
use yarnspinner::runtime::{CHARACTER_ATTRIBUTE, CHARACTER_ATTRIBUTE_NAME_PROPERTY};

pub(crate) fn localized_line_plugin(_app: &mut App) {}

/// A line from the Yarn file, with all metadata and markup parsed.
/// The text is localized according to the localization logic used by the [`TextProvider`].
#[derive(Debug, Clone, PartialEq)]
pub struct LocalizedLine {
    /// The ID of the line in the string table.
    pub id: LineId,
    /// The original text, with all parsed markers removed.
    pub text: String,
    /// The [`MarkupAttribute`]s in this line. An example of markup is `Hello, [b]world[/b]!`.
    pub attributes: Vec<MarkupAttribute>,
    /// The list of metadata associated with this line, excluding the line ID.
    /// Metadata is defined by the hashtags at the end of the line, e.g. `Hello, world! #greeting #friendly`.
    /// This data is also provided in the `comment` field of a generated strings file.
    pub metadata: Vec<String>,
    /// The assets associated with this line, provided by [`AssetProvider`]s that were added with [`DialogueRunnerBuilder::add_asset_provider`].
    pub assets: LineAssets,
}
impl LocalizedLine {
    // Documentation taken from `YarnLine`
    /// Gets the first attribute with the specified name, if present.
    pub fn attribute(&self, name: &str) -> Option<&MarkupAttribute> {
        self.attributes.iter().find(|attr| attr.name == name)
    }

    // Documentation taken from `YarnLine`
    /// The name of the character, if present.
    /// ## Examples
    /// When there is a name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bevy_yarnspinner::prelude::*;
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
    /// #    metadata: vec![],
    /// #    assets: Default::default(),
    /// # };
    /// assert_eq!("Alice: Hello! How are you today?", line.text);
    /// assert_eq!(Some("Alice"), line.character_name());
    /// ```
    ///
    /// When there is no name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bevy_yarnspinner::prelude::*;
    /// # let line = LocalizedLine {
    /// #    id: "line".into(),
    /// #    text: "Great, thanks".to_owned(),
    /// #    attributes: vec![],
    /// #    metadata: vec![],
    /// #    assets: Default::default(),
    /// # };
    /// assert_eq!("Great, thanks", line.text);
    /// assert!(line.character_name().is_none());
    pub fn character_name(&self) -> Option<&str> {
        if let Some(attribute) = self.attribute(CHARACTER_ATTRIBUTE) {
            if let Some(name) = attribute.property(CHARACTER_ATTRIBUTE_NAME_PROPERTY) {
                let MarkupValue::String(name) = name else {
                    panic!(
                        "Attribute \"character\" has a \"name\" property, but it is not a string. \
                         This is a bug. Please report it at https://github.com/YarnSpinnerTool/YarnSpinner-Rust/issues/new"
                    );
                };
                return Some(name.as_str());
            }
        }
        None
    }

    // Documentation taken from `YarnLine`
    /// The underlying text for this line, with any `character` attribute removed.
    ///
    /// ## Examples
    /// When there is a name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bevy_yarnspinner::prelude::*;
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
    /// #    metadata: vec![],
    /// #    assets: Default::default(),
    /// # };
    /// assert_eq!("Alice: Hello! How are you today?", line.text);
    /// assert_eq!("Hello! How are you today?", &line.text_without_character_name());
    /// ```
    ///
    /// When there is no name:
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bevy_yarnspinner::prelude::*;
    /// # let line = LocalizedLine {
    /// #    id: "line".into(),
    /// #    text: "Great, thanks".to_owned(),
    /// #    attributes: vec![],
    /// #    metadata: vec![],
    /// #    assets: Default::default(),
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

    // Documentation taken from `YarnLine`
    /// Returns the substring of [`YarnLine::text`] covered by the passed `attribute`s [`MarkupAttribute::position`] and [`MarkupAttribute::length`] fields.
    pub fn text_for_attribute(&self, attribute: &MarkupAttribute) -> &str {
        assert!(
            self.text.len() <= attribute.position + attribute.length,
            "Attribute \"{attribute}\" represents a range not representable by this text: \"{}\". \
        Does this MarkupAttribute belong to this MarkupParseResult?",
            self.text
        );
        &self.text[attribute.position..attribute.position + attribute.length]
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
    /// Panics if `attribute_to_delete` is not an attribute of this [`YarnLine::attribute`].
    pub fn delete_range(&self, attribute_to_delete: &MarkupAttribute) -> Self {
        let yarn_line: YarnLine = self.clone().into();
        let deleted_range = yarn_line.delete_range(attribute_to_delete);
        Self::from_yarn_line(deleted_range, self.assets.clone(), self.metadata.clone())
    }

    /// Returns `true` if this line comes right before an options block.
    ///
    /// "right before" means that no commands are called in between them, no variables are set, etc., in which case this returns `false`.
    pub fn is_last_line_before_options(&self) -> bool {
        self.metadata.iter().any(|m| m == "lastline")
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

impl LocalizedLine {
    pub(crate) fn from_yarn_line(
        line: YarnLine,
        assets: LineAssets,
        metadata: Vec<String>,
    ) -> Self {
        Self {
            id: line.id,
            text: line.text,
            attributes: line.attributes,
            metadata,
            assets,
        }
    }
}
