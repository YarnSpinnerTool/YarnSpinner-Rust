use crate::markup::markup_parse_result::{
    AttributeMarkerProcessor, MarkupAttributeMarker, MarkupValue,
};
use std::collections::HashMap;

pub(crate) const REPLACEMENT_MARKER_CONTENTS: &str = "contents";

#[derive(Default, Debug, Clone)]
pub(crate) struct LineParser<'a> {
    marker_processors: HashMap<&'a str, &'a dyn AttributeMarkerProcessor>,
}

impl<'a> LineParser<'a> {
    pub(crate) fn add_marker_processor(
        &mut self,
        attribute_name: &'a str,
        processor: &'a dyn AttributeMarkerProcessor,
    ) {
        self.marker_processors.insert(attribute_name, processor);
    }
}

impl<'a> Default for LineParser<'a> {
    fn default() -> Self {
        Self {
            // Implementation note: See constructor in C#.
            marker_processors: HashMap::from([("nomarkup", NoMarkupTextProcessor::new())]),
        }
    }
}

const EMPTY_STRING_MARKUP_VALUE: MarkupValue = MarkupValue::String("".to_owned());

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
}
