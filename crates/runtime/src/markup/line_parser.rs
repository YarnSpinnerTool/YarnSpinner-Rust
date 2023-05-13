//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/LineParser.cs>

use crate::markup::markup_parse_result::ParsedMarkup;
use crate::markup::{
    AttributeMarkerProcessor, MarkupAttribute, MarkupAttributeMarker, MarkupParseError,
    MarkupValue, NoMarkupTextProcessor, TagType,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub type Result<T> = std::result::Result<T, MarkupParseError>;

#[derive(Debug, Clone)]
pub(crate) struct LineParser {
    // ## Implementation notes
    // We don't port `stringReader` because [`BufReader`] is not [`Clone`]
    /// A map for the names of attributes to an object that can generate replacement text for those attributes.
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
        if input.len() == 0 {
            // We got a null input; return an empty markup parse result
            return Ok(ParsedMarkup::new());
        }

        self.input = normalize(input);
        self.source_position = 0;

        let mut text = String::new();
        let mut markers = Vec::new();
        let mut last_character = 0 as char;

        // Read the entirety of the line
        while let Some(character) = self.read_char() {
            match character {
                '\\' => {
                    // This may be the start of an escaped bracket ("\[" or "\]"). Peek ahead to see if it is.
                    if let Some(next_character) = self.peek_char() {
                        if next_character == '[' || next_character == ']' {
                            // It is! We'll discard this '\', and read the next character as plain text.
                            let character = self.read_char().unwrap();
                            text.push(character);
                            continue;
                        } else {
                            // It wasn't an escaped bracket. Continue on, and parse the '\' as a normal character.
                        }
                    }
                }
                '[' => {
                    // How long is our current string, in text elements (i.e. visible glyphs)?
                    self.position = UnicodeSegmentation::graphemes(text.as_str(), true).count();

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
                        let replacement_text = self.process_replacement_marker(&mut marker);

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
                            self.read_char();
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

        let mut attributes = self.build_attributes_from_markers(markers);
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

    pub(crate) fn set_language_code(&mut self, language_code: impl Into<String>) {
        let language_code = language_code.into();
        for processor in self.marker_processors.values_mut() {
            processor.set_language_code(language_code.clone());
        }
    }

    /// Parses an open, close, self-closing, or close-all attribute marker.
    fn parse_attribute_marker(&mut self) -> Result<MarkupAttributeMarker> {
        let source_position_at_marker_start = self.source_position;
        // We have already consumed the start of the marker '[' before
        // we enter here. Increment the sourcePosition counter to
        // account for it.
        self.source_position += 1;

        // Next, start parsing from the characters that can appear
        // inside the marker
        if let Some('/') = self.peek_char() {
            // This is either the start of a closing tag or the start
            // of the 'close-all' tag
            self.parse_character('/')?;
            if let Some(']') = self.peek_char() {
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

        let mut properties: HashMap<String, MarkupValue> = HashMap::new();

        // If the ID was immediately followed by an '=', this was the
        // first property (its value is also used as the attribute
        // name.)
        todo!()
    }

    /// Parses a marker and generates replacement text to insert into the plain text.
    ///
    /// ## Returns
    ///
    /// The replacement text to insert.
    fn process_replacement_marker(&self, marker: &mut MarkupAttributeMarker) -> String {
        todo!()
    }

    /// Peeks ahead in the input without consuming any characters, looking for whitespace.
    ///
    /// This method returns [`None`] if the parser has reached the end of the line.
    fn peek_whitespace(&self) -> Option<bool> {
        self.peek_char().map(|character| character.is_whitespace())
    }

    /// Creates a list of [`MarkupAttribute`]s from loose [MarkupAttributeMarker`]s
    ///
    /// ## Panics
    ///
    /// Panics when a close marker is encountered, but no corresponding open marker for it exists.
    fn build_attributes_from_markers(
        &self,
        _markers: Vec<MarkupAttributeMarker>,
    ) -> Vec<MarkupAttribute> {
        todo!()
    }

    fn read_char(&mut self) -> Option<char> {
        let character = self.input.chars().nth(self.source_position);
        self.source_position += 1;
        character
    }

    fn peek_char(&self) -> Option<char> {
        todo!("This should sometimes eat whitespace, sometimes not. See the difference between this.stringReader.foo and this.foo"); // I checked `read_char` and that one's okay! :) it "counterpart" is `parse_character`
        self.input.chars().nth(self.source_position)
    }

    fn parse_character(&mut self, character: char) -> Result<()> {
        self.consume_whitespace()?;
        let next = self
            .read_char()
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
                self.peek_char()
                    .ok_or_else(|| MarkupParseError::UnexpectedWhitespaceEnd {
                        input: self.input.clone(),
                    })?;
            if !next.is_whitespace() {
                // no more whitespace ahead; don't consume it, but
                // instead stop eating whitespace
                break;
            }
            self.read_char().unwrap();
        }
        Ok(())
    }

    fn parse_id(&self) -> Result<String> {
        todo!()
    }
}

/// Returns a new string whose textual value is the same as this string, but whose binary representation is in Unicode normalization form C.
fn normalize(string: &str) -> String {
    string.nfc().to_string()
}

/// The name of the property in replacement attributes that contains the text of the attribute.
pub(crate) const REPLACEMENT_MARKER_CONTENTS: &str = "contents";

/// The name of the implicitly-generated `character` attribute.
pub(crate) const CHARACTER_ATTRIBUTE: &str = "character";

/// The name of the 'name' property, on the implicitly-generated `character` attribute.
pub(crate) const CHARACTER_ATTRIBUTE_NAME_PROPERTY: &str = "name";

/// The name of the property to use to signify that trailing whitespace should be trimmed
/// if a tag had preceding whitespace or begins the line. This property must be a bool value.
pub(crate) const TRIM_WHITESPACE_PROPERTY: &str = "trimwhitespace";

/// A regular expression that matches a colon followed by optional whitespace.
static END_OF_CHARACTER_MARKER: Lazy<Regex> = Lazy::new(|| Regex::new(r":\s*").unwrap());
