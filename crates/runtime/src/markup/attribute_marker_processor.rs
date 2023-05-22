//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/IAttributeMarkerProcessor.cs>

pub(crate) use self::{dialogue_text_processor::*, no_markup_text_processor::*};
use crate::markup::MarkupAttributeMarker;
use crate::prelude::Language;
use core::fmt::Debug;

mod dialogue_text_processor;
mod no_markup_text_processor;

/// Provides a mechanism for producing replacement text for a marker.
pub(crate) trait AttributeMarkerProcessor: Debug + Send + Sync {
    /// Produces the replacement text that should be inserted into a parse
    /// result for a given attribute.
    ///
    /// If the marker is an `open` marker, the text from the marker's
    /// position to its corresponding closing marker is provided as a string
    /// property called `contents`.
    fn replacement_text_for_marker(&self, marker: &MarkupAttributeMarker) -> String;
    fn set_language_code(&mut self, language_code: Language);
    fn clone_box(&self) -> Box<dyn AttributeMarkerProcessor>;
}

impl Clone for Box<dyn AttributeMarkerProcessor> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
