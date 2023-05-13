use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarkupParseError {
    #[error("Error parsing line {input}: attribute {name:?} at position {position} has a {type_} property \"{prop}\" - this property is required to be a boolean value.")]
    TrimWhitespaceAttributeIsNotBoolean {
        input: String,
        name: Option<String>,
        position: usize,
        type_: String,
        prop: String,
    },
}
