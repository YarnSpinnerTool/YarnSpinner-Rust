use crate::markup::{MarkupAttributeMarker, TRIM_WHITESPACE_PROPERTY};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarkupParseError {
    #[error("Error parsing line {input}: attribute {name:?} at position {position} has a {type_} property \"{TRIM_WHITESPACE_PROPERTY}\" - this property is required to be a boolean value.")]
    TrimWhitespaceAttributeIsNotBoolean {
        input: String,
        name: Option<String>,
        position: usize,
        type_: String,
    },
    #[error("Line ended when expecting whitespace instead: \"{input}\"")]
    UnexpectedWhitespaceEnd { input: String },
    #[error("Unexpected end of line inside markup in line \"{input}\"")]
    UnexpectedEndOfLine { input: String },
    #[error("Expected a {character} inside markup in line \"{input}\"")]
    UnexpectedCharacter { input: String, character: char },
    #[error(
        "Unexpected close marker {name:?} at position {position} in line {input}",
        name = marker.name,
        position = marker.position,
    )]
    UnmatchedCloseMarker {
        input: String,
        marker: MarkupAttributeMarker,
    },
}
