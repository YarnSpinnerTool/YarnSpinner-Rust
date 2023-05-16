use crate::markup::TRIM_WHITESPACE_PROPERTY;
use crate::prelude::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
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
    #[error("Unexpected close marker {name} at position {position} in line {input}")]
    UnmatchedCloseMarker {
        input: String,
        name: String,
        position: usize,
    },
    #[error("Expected an identifier inside markup in line \"{input}\"")]
    NoIdentifierFound { input: String },
    #[error("Expected a string inside markup in line \"{input}\"")]
    NoStringFound { input: String },
    #[error("Invalid escaped character in line \"{input}\"")]
    InvalidEscapeSequence { input: String },
    #[error("Unterminated marker {name} in line {input} at position {position}")]
    UnterminatedMarker {
        input: String,
        name: String,
        position: usize,
    },
}
