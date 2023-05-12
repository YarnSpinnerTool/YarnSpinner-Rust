//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/LineParser.cs>

use crate::markup::markup_parse_result::MarkupParseResult;
use crate::markup::{AttributeMarkerProcessor, NoMarkupTextProcessor};
use std::collections::HashMap;

pub(crate) const REPLACEMENT_MARKER_CONTENTS: &str = "contents";

#[derive(Debug, Clone)]
pub(crate) struct LineParser<'a> {
    marker_processors: HashMap<&'a str, Box<dyn AttributeMarkerProcessor>>,
}

impl<'a> LineParser<'a> {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add_marker_processor(
        &mut self,
        attribute_name: &'a str,
        processor: impl AttributeMarkerProcessor + 'static,
    ) {
        let inserted = self
            .marker_processors
            .insert(attribute_name, Box::new(processor));
        assert!(inserted.is_none(), "A marker processor for the attribute '{attribute_name}' has already been added. This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new");
    }

    pub(crate) fn parse_markup(&self, input: &str) -> MarkupParseResult {
        if input.len() == 0 {
            return MarkupParseResult::default();
        }

        let original_text = normalize_to_form_c(input);
        todo!()
    }
}

/// Returns a new string whose textual value is the same as this string, but whose binary representation is in Unicode normalization form C.
fn normalize_to_form_c(_: &str) {
    // Use https://docs.rs/unicode-normalization/latest/unicode_normalization/trait.UnicodeNormalization.html#tymethod.nfc
    todo!()
}

impl<'a> Default for LineParser<'a> {
    fn default() -> Self {
        Self {
            marker_processors: HashMap::from([(
                "nomarkup",
                Box::new(NoMarkupTextProcessor::new()) as Box<dyn AttributeMarkerProcessor>,
            )]),
        }
    }
}
