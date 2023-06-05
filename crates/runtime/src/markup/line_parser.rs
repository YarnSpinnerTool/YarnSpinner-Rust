//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/LineParser.cs>

use crate::markup::parsed_markup::ParsedMarkup;
use crate::markup::{
    AttributeMarkerProcessor, MarkupAttribute, MarkupAttributeMarker, MarkupParseError,
    MarkupValue, NoMarkupTextProcessor, TagType,
};
use crate::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub type Result<T> = std::result::Result<T, MarkupParseError>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub(crate) struct LineParser {
    // ## Implementation notes
    // We don't port `stringReader` because [`BufReader`] is not [`Clone`]
    /// A map for the names of attributes to an object that can generate replacement text for those attributes.
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    #[cfg_attr(feature = "serde", serde(skip))]
    marker_processors: HashMap<String, Box<dyn AttributeMarkerProcessor>>,
    /// The original text that this line parser is parsing.
    input: String,
    /// The current position of the string reader in the plain text, measured in characters.
    source_position: usize,
    /// The current position of the string reader in the plain text, measured in text elements.
    position: usize,
}

impl Default for LineParser {
    fn default() -> Self {
        Self {
            marker_processors: HashMap::from([(
                "nomarkup".to_string(),
                Box::new(NoMarkupTextProcessor::new()) as Box<dyn AttributeMarkerProcessor>,
            )]),
            input: Default::default(),
            source_position: Default::default(),
            position: Default::default(),
        }
    }
}

impl LineParser {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Registers an object as a marker processor for a given
    /// marker name.
    ///
    /// When a marker processor is registered for a marker name, the
    /// parser will ask the processor for text to insert into the plain
    /// text. This allows users of the [`LineParser`]
    /// to dynamically replace text in a line. The `nomarkup` tag is
    /// implemented in this way by the [`LineParser`]
    /// directly; the [`Dialogue`] uses this mechanism
    /// to implement the `select`, `plural` and `ordinal` markers.
    pub(crate) fn register_marker_processor(
        mut self,
        attribute_name: impl Into<String>,
        processor: Box<dyn AttributeMarkerProcessor>,
    ) -> Self {
        let attribute_name = attribute_name.into();
        let previous_value = self
            .marker_processors
            .insert(attribute_name.clone(), processor);
        assert!(previous_value.is_none(),
                "A marker processor for the attribute '{attribute_name}' has already been added. \
                This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new");
        self
    }

    /// Parses a line of text, and produces a [`ParsedMarkup`] containing the processed text
    ///
    /// ## Implementation notes
    ///
    /// The original does not reset the internal `source_position`. This was likely a bug.
    pub(crate) fn parse_markup(&mut self, input: &str) -> Result<ParsedMarkup> {
        if input.is_empty() {
            // We got a null input; return an empty markup parse result
            return Ok(ParsedMarkup::new());
        }

        self.input = normalize(input);
        self.source_position = 0;

        let mut text = String::new();
        let mut markers = Vec::new();
        let mut last_character = 0 as char;

        // Read the entirety of the line
        while let Some(character) = self.read_next() {
            match character {
                '\\' => {
                    // This may be the start of an escaped bracket ("\[" or "\]"). Peek ahead to see if it is.
                    if let Some(next_character) = self.peek_next() {
                        if next_character == '[' || next_character == ']' {
                            // It is! We'll discard this '\', and read the next character as plain text.
                            let character = self.read_next().unwrap();
                            text.push(character);
                            continue;
                        }
                    }
                    // It wasn't an escaped bracket. Continue on, and parse the '\' as a normal character.
                    text.push(character);
                }
                '[' => {
                    // How long is our current string, in text elements (i.e. visible glyphs)?
                    self.position = text.as_str().graphemes(true).count();

                    // The start of a marker!
                    let mut marker = self.parse_attribute_marker()?;

                    let had_preceding_whitespace_or_line_start =
                        self.source_position == 0 || last_character.is_whitespace();

                    // Is this a replacement marker?
                    let was_replacement_marker = marker
                        .name
                        .as_ref()
                        .map(|name| self.marker_processors.contains_key(name))
                        .unwrap_or_default();
                    if was_replacement_marker {
                        // Process it and get the replacement text!
                        let replacement_text = self.process_replacement_marker(&mut marker)?;

                        // Insert it into our final string and update our position accordingly
                        text.push_str(&replacement_text);
                    }
                    let mut trim_whitespace_if_able = false;
                    if had_preceding_whitespace_or_line_start {
                        // By default, self-closing markers will trim a single trailing whitespace after it if there was preceding whitespace.
                        // This doesn't happen if the marker was a replacement marker, or it has a property "trimwhitespace" (which must be boolean) set to false.
                        // All markers can opt-in to trailing whitespace trimming by having a 'trimwhitespace' property set to true.
                        if marker.tag_type == TagType::SelfClosing {
                            trim_whitespace_if_able = !was_replacement_marker;
                        }
                        if let Some(prop) = marker.properties.get(TRIM_WHITESPACE_PROPERTY) {
                            let MarkupValue::Bool(trim_whitespace) = prop else {
                                return Err(MarkupParseError::TrimWhitespaceAttributeIsNotBoolean {
                                    input: self.input.clone(),
                                    name: marker.name,
                                    position: self.position,
                                    type_: prop.type_name().to_lowercase(),
                                });
                            };
                            trim_whitespace_if_able = *trim_whitespace;
                        }
                    }
                    if trim_whitespace_if_able {
                        // If there's trailing whitespace, and we want to remove it, do so
                        if let Some(true) = self.peek_whitespace() {
                            // Consume the single trailing whitespace character (and don't update position)
                            self.read_next();
                        }
                    }
                    markers.push(marker);
                }
                _ => {
                    // plain text! add it to the resulting string and
                    // advance the parser's plain-text position
                    text.push(character);
                }
            }

            last_character = character;
        }

        let mut attributes = self.build_attributes_from_markers(markers)?;
        let character_attribute_is_present = attributes
            .iter()
            .any(|attr| attr.name == CHARACTER_ATTRIBUTE);
        if character_attribute_is_present {
            return Ok(ParsedMarkup { text, attributes });
        }

        // Attempt to generate a character attribute from the start
        // of the string to the first colon
        let Some(match_) = END_OF_CHARACTER_MARKER.find(&self.input) else {
            return Ok(ParsedMarkup { text, attributes });
        };

        let character_name = self.input[..match_.start()].to_string();

        let character_attribute = MarkupAttribute {
            name: CHARACTER_ATTRIBUTE.to_string(),
            position: 0,
            length: match_.end(),
            properties: HashMap::from([(
                CHARACTER_ATTRIBUTE_NAME_PROPERTY.to_string(),
                character_name.into(),
            )]),
            source_position: 0,
        };

        attributes.push(character_attribute);
        Ok(ParsedMarkup { text, attributes })
    }

    pub(crate) fn set_language_code(&mut self, language_code: impl Into<Option<Language>>) {
        let language_code = language_code.into();
        for processor in self.marker_processors.values_mut() {
            processor.set_language_code(language_code.clone());
        }
    }

    /// Parses an open, close, self-closing, or close-all attribute marker.
    fn parse_attribute_marker(&mut self) -> Result<MarkupAttributeMarker> {
        // Implementation note: -1 because the original increments `source_position` at the end of the loop in `parse_markup`,
        // while we do it at the beginning instead
        let source_position_at_marker_start = self.source_position - 1;

        // Implementation note: No need to advance position here,
        // since we do so automatically on every read / parse_character

        // Next, start parsing from the characters that can appear
        // inside the marker
        if self.peek_character('/')? {
            // This is either the start of a closing tag or the start
            // of the 'close-all' tag
            self.parse_character('/')?;
            if self.peek_character(']')? {
                // It's the close-all tag!
                self.parse_character(']')?;
                return Ok(MarkupAttributeMarker {
                    tag_type: TagType::CloseAll,
                    name: None,
                    properties: HashMap::new(),
                    position: self.position,
                    source_position: source_position_at_marker_start,
                });
            }
            // It's a named closing tag!
            let tag_name = self.parse_id()?;
            self.parse_character(']')?;
            return Ok(MarkupAttributeMarker {
                tag_type: TagType::Close,
                name: Some(tag_name),
                properties: HashMap::new(),
                position: self.position,
                source_position: source_position_at_marker_start,
            });
        }

        // If we're here, this is either an opening tag, or a
        // self-closing tag.

        // If the opening ID is not provided, the name of the attribute
        // is taken from the first property.

        // Tags always start with an ID, which is used as the name of
        // the attribute.
        let attribute_name = self.parse_id()?;

        let mut properties = HashMap::new();

        // If the ID was immediately followed by an '=', this was the
        // first property (its value is also used as the attribute
        // name.)
        if self.peek_character('=')? {
            // This is also the first property!

            // Parse the rest of the property now before we parse any
            // others.
            self.parse_character('=')?;
            let value = self.parse_value()?;
            properties.insert(attribute_name.clone(), value);
        }

        // parse all remaining properties
        loop {
            self.consume_whitespace()?;
            let next = self
                .peek_next()
                .ok_or_else(|| MarkupParseError::UnexpectedEndOfLine {
                    input: self.input.clone(),
                })?;
            match next {
                ']' => {
                    // End of an Opening tag.
                    self.parse_character(']')?;
                    return Ok(MarkupAttributeMarker {
                        tag_type: TagType::Open,
                        name: Some(attribute_name),
                        properties,
                        position: self.position,
                        source_position: source_position_at_marker_start,
                    });
                }
                '/' => {
                    // End of a self-closing tag.
                    self.parse_character('/')?;
                    self.parse_character(']')?;
                    return Ok(MarkupAttributeMarker {
                        tag_type: TagType::SelfClosing,
                        name: Some(attribute_name),
                        properties,
                        position: self.position,
                        source_position: source_position_at_marker_start,
                    });
                }
                _ => {
                    // Expect another property.
                    let property_name = self.parse_id()?;
                    self.parse_character('=')?;
                    let value = self.parse_value()?;
                    properties.insert(property_name, value);
                }
            }
        }
    }

    /// Parses a marker and generates replacement text to insert into the plain text.
    ///
    /// ## Returns
    ///
    /// The replacement text to insert.
    fn process_replacement_marker(&mut self, marker: &mut MarkupAttributeMarker) -> Result<String> {
        let name = marker.name.as_ref().unwrap().as_str();
        match marker.tag_type {
            TagType::Open => {
                // this is an attribute that we want to replace with text!

                // if this is an opening marker, we read up to the closing
                // marker, the close-all marker, or the end of the string; this
                // becomes the value of a property called "contents", and then
                // we perform the replacement

                // Read everything up to the closing tag
                let marker_contents = self.parse_raw_text_up_to_attribute_close(name)?;

                // Add this as a property
                marker.properties.insert(
                    REPLACEMENT_MARKER_CONTENTS.to_string(),
                    marker_contents.into(),
                );
            }
            TagType::SelfClosing => {}
            TagType::CloseAll | TagType::Close => {
                // If it's not an open or self-closing marker, we have no text
                // to insert, so return the empty string
                return Ok(String::new());
            }
        }
        // Fetch the text that should be inserted into the string at
        // this point
        let replacement = self
            .marker_processors
            .get(name)
            .unwrap()
            .replacement_text_for_marker(marker);
        Ok(replacement)
    }

    /// Peeks ahead in the input without consuming any characters, looking for whitespace.
    ///
    /// This method returns [`None`] if the parser has reached the end of the line.
    fn peek_whitespace(&self) -> Option<bool> {
        self.peek_next().map(|character| character.is_whitespace())
    }

    /// Creates a list of [`MarkupAttribute`]s from loose [MarkupAttributeMarker`]s
    ///
    /// ## Retuns
    ///
    /// Returns an `Err` when a close marker is encountered, but no corresponding open marker for it exists.
    fn build_attributes_from_markers(
        &self,
        markers: Vec<MarkupAttributeMarker>,
    ) -> Result<Vec<MarkupAttribute>> {
        let mut unclosed_markers = VecDeque::new();
        let mut attributes = Vec::with_capacity(markers.len());
        for marker in markers {
            match marker.tag_type {
                TagType::Open => {
                    // A new marker! Add it to the unclosed list at the
                    // start (because there's a high chance that it
                    // will be closed soon).
                    unclosed_markers.push_front(marker);
                }
                TagType::Close => {
                    // A close marker! Walk back through the
                    // unclosed stack to find the most recent
                    // marker of the same type to find its pair.
                    assert!(marker.name.is_some());
                    let matched_open_marker_index = unclosed_markers
                        .iter()
                        .position(|open_marker| open_marker.name == marker.name)
                        .ok_or_else(|| MarkupParseError::UnmatchedCloseMarker {
                            input: self.input.clone(),
                            name: marker.name.unwrap(),
                            position: marker.position,
                        })?;

                    // This attribute is now closed, so we can
                    // remove the marker from the unmatched list
                    let matched_open_marker =
                        unclosed_markers.remove(matched_open_marker_index).unwrap();

                    // We can now construct the attribute!
                    let length = marker.position - matched_open_marker.position;
                    let attribute = MarkupAttribute::from_marker(matched_open_marker, length);
                    attributes.push(attribute);
                }
                TagType::SelfClosing => {
                    // Self-closing markers create a zero-length
                    // attribute where they appear
                    let attribute = MarkupAttribute::from_marker(marker, 0);
                    attributes.push(attribute);
                }
                TagType::CloseAll => {
                    // Close all currently open markers

                    // For each marker that we currently have open,
                    // this marker has closed it, so create an
                    // attribute for it
                    let attributes_to_add = unclosed_markers.iter().map(|open_marker| {
                        let length = marker.position - open_marker.position;
                        MarkupAttribute::from_marker(open_marker.clone(), length)
                    });
                    attributes.extend(attributes_to_add);

                    // We've now closed all markers, so we can
                    // clear the unclosed list now
                    unclosed_markers.clear();
                }
            }
        }

        attributes.sort_by_key(|attribute| attribute.source_position);
        Ok(attributes)
    }

    fn read_next(&mut self) -> Option<char> {
        let character = self.input.chars().nth(self.source_position);
        self.source_position += 1;
        character
    }

    fn peek_next(&self) -> Option<char> {
        self.input.chars().nth(self.source_position)
    }

    fn read_to_end(&mut self) -> String {
        let mut string = String::new();
        while let Some(next) = self.read_next() {
            string.push(next);
        }
        string
    }

    fn peek_character(&mut self, character: char) -> Result<bool> {
        self.consume_whitespace()?;
        let match_ = self.peek_next() == Some(character);
        Ok(match_)
    }

    fn parse_character(&mut self, character: char) -> Result<()> {
        self.consume_whitespace()?;
        let next = self
            .read_next()
            .ok_or_else(|| MarkupParseError::UnexpectedEndOfLine {
                input: self.input.clone(),
            })?;
        if next != character {
            return Err(MarkupParseError::UnexpectedCharacter {
                input: self.input.clone(),
                character,
            });
        }
        Ok(())
    }

    /// Reads and discards whitespace, up to the first non-whitespace
    /// character.
    ///
    /// ## Returns
    ///
    /// Returns `Err` when the end of the line is reached while consuming.
    ///
    /// ## Implementation notes
    /// `allowEndOfLine` was not ported because it was always `false`
    fn consume_whitespace(&mut self) -> Result<()> {
        loop {
            let next =
                self.peek_next()
                    .ok_or_else(|| MarkupParseError::UnexpectedWhitespaceEnd {
                        input: self.input.clone(),
                    })?;
            if !next.is_whitespace() {
                // no more whitespace ahead; don't consume it, but
                // instead stop eating whitespace
                break;
            }
            self.read_next().unwrap();
        }
        Ok(())
    }

    fn parse_id(&mut self) -> Result<String> {
        self.consume_whitespace()?;
        let mut id = String::new();

        // Read the first character, which must be a letter, number, or underscore
        let next = self
            .read_next()
            .ok_or_else(|| MarkupParseError::UnexpectedEndOfLine {
                input: self.input.clone(),
            })?;

        // Implementation notes: no surrogate checks because UTF-16 surrogates are not valid Rust chars
        // See <https://github.com/rust-lang/rust/issues/94919>
        if next.is_alphanumeric() || next == '_' {
            id.push(next);
        } else {
            return Err(MarkupParseError::NoIdentifierFound {
                input: self.input.clone(),
            });
        }

        // Read zero or more letters, numbers, or underscores
        while let Some(next) = self.peek_next() {
            // Implementation notes: again, no surrogate checks. See above comment.
            if next.is_alphanumeric() || next == '_' {
                id.push(next);
                // consume it
                self.read_next().unwrap();
            } else {
                // no more
                break;
            }
        }

        Ok(id)
    }

    /// Parses a property value.
    ///
    /// Permitted value types are:
    ///
    /// - Integers
    /// - Floating-point numbers
    /// - Strings (delimited by double quotes). (Strings may contain
    /// escaped quotes with a backslash.)
    /// - The words `true` or `false`
    /// - Runs of alphanumeric characters, up to but not including a
    /// whitespace or the end of a tag; these are interpreted as a string
    /// (e.g. `[mood=happy]` is interpreted the same as `[mood="happy"]`
    /// - Expressions (delimited by curly braces), which are processed
    /// as inline expressions.
    fn parse_value(&mut self) -> Result<MarkupValue> {
        // parse integers or floats:
        if self.peek_numeric()? {
            // could be an int or a float
            let integer = self.parse_integer()?;

            // if there's a decimal separator, this is a float
            if !self.peek_character('.')? {
                // no decimal separator, so this is an integer
                return Ok(integer.into());
            }

            // a float
            self.parse_character('.')?;

            // parse the fractional value
            let fraction = self.parse_integer()?;
            let float: f32 = format!("{integer}.{fraction}").parse().unwrap();
            return Ok(float.into());
        }
        if self.peek_character('"')? {
            // A string
            let string = self.parse_string()?;
            return Ok(string.into());
        }

        let word = self.parse_id()?;

        // This ID is expected to be 'true', 'false', or something
        // else. if it's 'true' or 'false', interpret it as a bool.
        match word.as_str() {
            "true" => Ok(true.into()),
            "false" => Ok(false.into()),
            // interpret this as a one-word string
            _ => Ok(word.into()),
        }
    }

    /// Parses text up to either a close marker with the given name, or
    /// a close-all marker.
    ///
    /// The closing marker itself is not included in the returned text.
    fn parse_raw_text_up_to_attribute_close(&mut self, name: &str) -> Result<String> {
        let original_source_position = self.source_position;
        let remainder_of_line = self.read_to_end();

        // Parse up to either [/name] or [/], allowing whitespace between any elements.
        let regex = Regex::new(&format!(r"\[\s*\/\s*({name})?\s*\]")).unwrap();
        let match_ =
            regex
                .find(&remainder_of_line)
                .ok_or_else(|| MarkupParseError::UnterminatedMarker {
                    input: self.input.clone(),
                    name: name.to_string(),
                    position: self.position,
                })?;

        // Split the line into the part up to the closing tag, and the
        // part afterwards
        let close_marker_position = match_.start();
        let raw_text_substring = &remainder_of_line[..close_marker_position];

        // We've consumed all of this text in the string reader, so to
        // make it possible to parse the rest, we need to create a new
        // "string reader" with the remaining text
        self.source_position = original_source_position + close_marker_position;

        Ok(raw_text_substring.to_string())
    }

    /// Peeks ahead in the LineParser's input without consuming any
    /// characters, looking for (ASCII) numeric characters.
    ///
    /// This method returns false if the parser has reached the end of
    /// the line.
    fn peek_numeric(&mut self) -> Result<bool> {
        self.consume_whitespace()?;
        let Some(next) = self.peek_next() else {
            return Ok(false)
        };
        Ok(next.is_ascii_digit())
    }

    /// Parses an (ASCII) integer from the stream.
    ///
    /// This method returns false if the parser has reached the end of
    /// the line.
    fn parse_integer(&mut self) -> Result<u32> {
        self.consume_whitespace()?;
        let mut integer_string = String::new();
        loop {
            let next = self
                .peek_next()
                .ok_or_else(|| MarkupParseError::UnexpectedEndOfLine {
                    input: self.input.clone(),
                })?;
            if next.is_ascii_digit() {
                self.read_next().unwrap();
                integer_string.push(next);
            } else {
                // end of the integer! parse and return it
                let integer = integer_string.parse().unwrap();
                return Ok(integer);
            }
        }
    }

    fn parse_string(&mut self) -> Result<String> {
        self.consume_whitespace()?;

        let mut string = String::new();

        let next = self
            .read_next()
            .ok_or_else(|| MarkupParseError::UnexpectedEndOfLine {
                input: self.input.clone(),
            })?;

        if next != '"' {
            return Err(MarkupParseError::NoStringFound {
                input: self.input.clone(),
            });
        }
        loop {
            let next = self
                .read_next()
                .ok_or_else(|| MarkupParseError::UnexpectedEndOfLine {
                    input: self.input.clone(),
                })?;
            match next {
                '"' => {
                    // end of string - consume it but don't append to the final collection
                    return Ok(string);
                }
                '\\' => {
                    // an escaped quote or backslash
                    let next =
                        self.read_next()
                            .ok_or_else(|| MarkupParseError::UnexpectedEndOfLine {
                                input: self.input.clone(),
                            })?;
                    if next == '"' || next == '\\' {
                        string.push(next);
                    } else {
                        // Implementation note:
                        // Not an error in the original implementation, but that seems like an oversight.
                        return Err(MarkupParseError::InvalidEscapeSequence {
                            input: self.input.clone(),
                        });
                    }
                }
                _ => {
                    string.push(next);
                }
            }
        }
    }
}

/// Returns a new string whose textual value is the same as this string, but whose binary representation is in Unicode normalization form C.
pub(crate) fn normalize(string: &str) -> String {
    string.nfc().to_string()
}

/// The name of the property in replacement attributes that contains the text of the attribute.
pub(crate) const REPLACEMENT_MARKER_CONTENTS: &str = "contents";

/// The name of the implicitly-generated `character` attribute.
pub const CHARACTER_ATTRIBUTE: &str = "character";

/// The name of the 'name' property, on the implicitly-generated `character` attribute.
pub const CHARACTER_ATTRIBUTE_NAME_PROPERTY: &str = "name";

/// The name of the property to use to signify that trailing whitespace should be trimmed
/// if a tag had preceding whitespace or begins the line. This property must be a bool value.
pub const TRIM_WHITESPACE_PROPERTY: &str = "trimwhitespace";

/// A regular expression that matches a colon followed by optional whitespace.
static END_OF_CHARACTER_MARKER: Lazy<Regex> = Lazy::new(|| Regex::new(r":\s*").unwrap());
