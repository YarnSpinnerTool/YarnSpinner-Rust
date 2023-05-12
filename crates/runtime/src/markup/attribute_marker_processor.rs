//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/IAttributeMarkerProcessor.cs>

use crate::markup::MarkupAttribute;
use core::fmt::Debug;

pub(crate) trait AttributeMarkerProcessor: Debug + Send + Sync {
    fn replacement_text_for_marker(&mut self, marker: &MarkupAttribute) -> String;
    fn clone_box(&self) -> Box<dyn AttributeMarkerProcessor>;
}

impl Clone for Box<dyn AttributeMarkerProcessor> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
