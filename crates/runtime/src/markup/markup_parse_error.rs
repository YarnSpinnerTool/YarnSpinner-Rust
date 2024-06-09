use crate::markup::TRIM_WHITESPACE_PROPERTY;
#[cfg(any(feature = "bevy", feature = "serde"))]
use crate::prelude::*;
use std::error::Error;
use std::fmt;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub enum MarkupParseError {
    TrimWhitespaceAttributeIsNotBoolean {
        input: String,
        name: Option<String>,
        position: usize,
        type_: String,
    },
    UnexpectedWhitespaceEnd {
        input: String,
    },
    UnexpectedEndOfLine {
        input: String,
    },
    UnexpectedCharacter {
        input: String,
        character: char,
    },
    UnmatchedCloseMarker {
        input: String,
        name: String,
        position: usize,
    },
    NoIdentifierFound {
        input: String,
    },
    NoStringFound {
        input: String,
    },
    InvalidEscapeSequence {
        input: String,
    },
    UnterminatedMarker {
        input: String,
        name: String,
        position: usize,
    },
}

impl Error for MarkupParseError {}

impl fmt::Display for MarkupParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MarkupParseError::*;
        match self {
            TrimWhitespaceAttributeIsNotBoolean {
                input,
                name,
                position,
                type_,
            } => write!(f, "Error parsing line {input}: attribute {name:?} at position {position} has a {type_} property \"{TRIM_WHITESPACE_PROPERTY}\" - this property is required to be a boolean value."),
            UnexpectedWhitespaceEnd { input } => write!(f, "Line ended when expecting whitespace instead: \"{input}\""),
            UnexpectedEndOfLine { input } => write!(f, "Unexpected end of line inside markup in line \"{input}\""),
            UnexpectedCharacter { input, character } => write!(f, "Expected a {character} inside markup in line \"{input}\""),
            UnmatchedCloseMarker {
                input,
                name,
                position,
            } => write!(f, "Unexpected close marker {name} at position {position} in line {input}"),
            NoIdentifierFound { input } => write!(f, "Expected an identifier inside markup in line \"{input}\""),
            NoStringFound { input } => write!(f, "Expected a string inside markup in line \"{input}\""),
            InvalidEscapeSequence { input } => write!(f, "Invalid escaped character in line \"{input}\""),
            UnterminatedMarker {
                input,
                name,
                position,
            } => write!(f, "Unterminated marker {name} in line {input} at position {position}"),
        }
    }
}
