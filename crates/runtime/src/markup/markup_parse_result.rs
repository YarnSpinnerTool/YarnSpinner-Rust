//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use yarn_slinger_core::prelude::*;

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

/// Represents a range of text in a marked-up string.
///
/// You do not create instances of this struct yourself.
/// It is created by objects that can parse markup, such as [`Dialogue`].
///
/// ## See also
/// - [`Dialogue::parse_markup`]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MarkupAttribute {
    /// The position in the plain text where this attribute begins.
    position: usize,
    /// The number of text elements in the plain text that this attribute covers.
    length: usize,
    /// The name of the attribute.
    name: String,
    /// The properties associated with this attribute.
    properties: HashMap<String, MarkupValue>,
    /// The position of the marker in the original text.
    source_position: usize,
}

/// A value associated with a [`MarkupProperty`]
///
/// You do not create instances of this struct yourself. It is created
/// by objects that can parse markup, such as [`Dialogue`]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MarkupValue {
    Integer(isize),
    Float(f32),
    String(String),
    Bool(bool),
}

pub(crate) trait AttributeMarkerProcessor: Debug + Send + Sync {
    fn replacement_text_for_marker(&mut self, marker: &MarkupAttribute) -> String;
    fn clone_box(&self) -> Box<dyn AttributeMarkerProcessor>;
}

impl Clone for Box<dyn AttributeMarkerProcessor> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TagType {
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
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MarkupProperty {
    name: String,
    value: MarkupValue,
}

/// Represents a marker (e.g. `[a]`) in line of marked up text.
///
/// You do not create instances of this struct yourself. It is created
/// by objects that can parse markup, such as [`Dialogue`].
pub(crate) struct MarkupAttributeMarker {
    /// The name of the marker.
    /// For example, the marker `[wave]` has the name `wave`.
    pub(crate) name: String,
    /// The position of the marker in the plain text.
    pub(crate) position: usize,
    /// The list of properties associated with this marker.
    pub(crate) properties: Vec<MarkupProperty>,
    /// The type of marker that this is.
    pub(crate) tag_type: TagType,
    /// The position of this marker in the original source text.
    pub(crate) source_position: usize,
}

pub trait MarkupPropertyVecExt {
    fn to_hash_map(self) -> HashMap<String, MarkupValue>;
}

impl MarkupPropertyVecExt for Vec<MarkupProperty> {
    fn to_hash_map(self) -> HashMap<String, MarkupValue> {
        let mut map = HashMap::new();
        for prop in self {
            map.insert(prop.name, prop.value);
        }
        map
    }
}
