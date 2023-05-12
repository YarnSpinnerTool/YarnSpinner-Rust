//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/NoMarkupTextProcessor.cs>

use crate::markup::{
    AttributeMarkerProcessor, MarkupAttributeMarker, MarkupValue, REPLACEMENT_MARKER_CONTENTS,
};

/// A markup text processor that implements the `[nomarkup]` attribute's behaviour.
#[derive(Default, Debug, Clone)]
pub(crate) struct NoMarkupTextProcessor;

impl NoMarkupTextProcessor {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl AttributeMarkerProcessor for NoMarkupTextProcessor {
    fn replacement_text_for_marker(&mut self, marker: &MarkupAttributeMarker) -> String {
        match marker.properties.get(REPLACEMENT_MARKER_CONTENTS) {
            Some(MarkupValue::String(v)) => v.to_owned(),
            // this is only possible when this marker is self-closing (i.e.
            // it's '[nomarkup/]'), in which case there's no text to
            // provide, so we'll provide the empty string here
            None => "".to_string(),
            _ => unreachable!("A NoMarkup marker contained something else then a string. This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new"),
        }
    }

    fn clone_box(&self) -> Box<dyn AttributeMarkerProcessor> {
        Box::new(self.clone())
    }
}
