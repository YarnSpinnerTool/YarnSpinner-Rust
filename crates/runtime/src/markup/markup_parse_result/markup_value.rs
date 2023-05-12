//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>
//! which was split into multiple files.

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
