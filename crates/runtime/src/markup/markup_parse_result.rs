//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>

use std::fmt::Debug;
use yarn_slinger_compiler::prelude::Position;

/// The result of parsing a line of marked-up text.
///
/// You do not create instances of this struct yourself. It is created
///  by objects that can parse markup, such as [`Dialogue`].
pub struct MarkupParseResult {
    /// The original text, with all parsed markers removed.
    pub text: String,
    /// The list of <see cref="MarkupAttribute"/>s in this parse result.
    pub attributes: Vec<MarkupAttribute>,
}

impl MarkupParseResult {
    pub fn new(text: String, attributes: Vec<MarkupAttribute>) -> Self {
        Self { text, attributes }
    }

    pub fn get_attribute_with_name(&self, name: &str) -> Option<&MarkupAttribute> {
        self.attributes.iter().find(|attr| attr.name == name)
    }

    /// Returns the substring of [`text`] covered by the [`attribute`]s Position and Length properties.
    ///
    /// ## Implementation notes:
    /// Instead of returning an empty string if the length is zero, we return none.
    pub fn text_for_attribute(&self, attribute: &MarkupAttribute) -> Option<&str> {
        if attribute.length == 0 {
            None
        } else {
            Some(&self.text[attribute.position..attribute.position + attribute.length])
        }
    }
}

/// A value associated with a `MarkupProperty`
///
/// You do not create instances of this struct yourself. It is created
/// by objects that can parse markup, such as [`Dialogue`]
///
/// # Implementation Notes
///
/// The original has a discriminator and 4 properties. It's obviously supposed to resemble a discriminated union.
// TODO: should we use YarnValue here? That one is missing integer, so we currently don't merge them.
pub enum MarkupValue {
    Integer(i32), // TODO: argue about size. In C# float(single) and int(32) are used.
    Float(f32),   // TODO: short is f16, but that doesnt even exist in rust?
    String(String),
    Bool(bool),
}

pub(crate) trait AttributeMarkerProcessor: Debug + Send + Sync {
    fn replacement_text_for_marker(&mut self, marker: &MarkupAttributeMarker) -> String;
    fn clone_box(&self) -> Box<dyn AttributeMarkerProcessor>;
}

impl Clone for Box<dyn AttributeMarkerProcessor> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub(crate) struct MarkupAttributeMarker {
    name: String,
    /// The position of the marker.
    position: Position,
    /// The position of the marker in the original text.
    source_position: Position,
    properties: Vec<MarkupProperty>, // TODO: maybe a hashset is smarter? It really should not have duplicates in name.
    marker_type: TagType,
}

impl MarkupAttributeMarker {
    pub fn get_property(&self, name: &str) -> Option<&MarkupValue> {
        self.properties
            .iter()
            .find(|prop| prop.name == name)
            .map(|prop| &prop.value)
    }
}

enum TagType {
    /// An open marker. For example, `[a]`.
    Open,

    /// A closing marker. For example, `[/a]`.
    Close,

    /// A self-closing marker. For example, `[a/]`.
    SelfClosing,

    /// The close-all marker, `[/]`.
    CloseAll,
}

/// A property associated with a `MarkupAttribute`.
///
/// You do not create instances of this struct yourself. It is created
/// by objects that can parse markup, such as [`Dialogue`]
struct MarkupProperty {
    name: String,
    value: MarkupValue,
}
