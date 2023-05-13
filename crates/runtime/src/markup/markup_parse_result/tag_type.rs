//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>
//! which was split into multiple files.

/// A type of [`MarkupAttributeMarker`].
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
