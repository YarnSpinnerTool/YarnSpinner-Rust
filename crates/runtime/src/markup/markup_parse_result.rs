//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>

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

trait AttributeMarkerProcessor {
    fn replacement_text_for_marker(&mut self, attribute_marker: &MarkupAttributeMarker) -> String;
}

struct MarkupAttributeMarker {
    name: String,
    /// The position of the marker.
    position: usize,
    /// The position of the marker in the original text.
    source_position: usize,
    properties: Vec<MarkupProperty>,
    marker_type: TagType,
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
