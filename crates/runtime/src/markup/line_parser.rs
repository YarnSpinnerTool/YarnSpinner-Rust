use crate::markup::markup_parse_result::{
    AttributeMarkerProcessor, MarkupAttributeMarker, MarkupParseResult, MarkupValue,
};
use std::collections::HashMap;

pub(crate) const REPLACEMENT_MARKER_CONTENTS: &str = "contents";

#[derive(Debug, Clone)]
pub(crate) struct LineParser<'a> {
    marker_processors: HashMap<&'a str, Box<dyn AttributeMarkerProcessor>>,
}

impl<'a> LineParser<'a> {
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

        let originalText = normalize_to_form_c(input);
        todo!()
    }
}

/// Returns a new string whose textual value is the same as this string, but whose binary representation is in Unicode normalization form C.
fn normalize_to_form_c() {
    // Use https://docs.rs/unicode-normalization/latest/unicode_normalization/trait.UnicodeNormalization.html#tymethod.nfc
    todo!()
}

impl<'a> Default for LineParser<'a> {
    fn default() -> Self {
        Self {
            // Implementation note: See constructor in C#.
            marker_processors: HashMap::from([(
                "nomarkup",
                Box::new(NoMarkupTextProcessor::default()) as Box<dyn AttributeMarkerProcessor>,
            )]),
        }
    }
}

/// A markup text processor that implements the `[nomarkup]` attribute's behaviour.
#[derive(Default, Debug, Clone)]
struct NoMarkupTextProcessor {}

impl AttributeMarkerProcessor for NoMarkupTextProcessor {
    fn replacement_text_for_marker(&mut self, marker: &MarkupAttributeMarker) -> String {
        match marker.get_property(REPLACEMENT_MARKER_CONTENTS) {
            Some(MarkupValue::String(v)) => v.to_owned(),
            // [sic] this is only possible when this marker is self-closing (i.e.
            // it's '[nomarkup/]'), in which case there's no text to
            // provide, so we'll provide the empty string here
            None => "".to_string(),
            _ => unreachable!("A NoMarkup marker may only contain string values. This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new"),
        }
    }

    fn clone_box(&self) -> Box<dyn AttributeMarkerProcessor> {
        Box::new(self.clone())
    }
}
