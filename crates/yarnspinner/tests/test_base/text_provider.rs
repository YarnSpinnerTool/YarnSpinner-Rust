use std::sync::{Arc, RwLock};
use yarnspinner_core::prelude::*;
use yarnspinner_runtime::prelude::*;

/// A [`TextProvider`] that wraps another [`TextProvider`] and is shallow cloned. It can thus be shared between users.
#[derive(Debug, Clone)]
pub struct SharedTextProvider(pub Arc<RwLock<Box<dyn TextProvider>>>);

impl SharedTextProvider {
    /// Creates a new [`SharedTextProvider`] that wraps the given [`TextProvider`].
    pub fn new(text_provider: impl TextProvider + 'static) -> Self {
        Self(Arc::new(RwLock::new(Box::new(text_provider))))
    }

    /// Replace the underlying [`TextProvider`] with another one. All copies of this [`SharedTextProvider`] will be affected.
    pub fn replace(&mut self, text_provider: impl TextProvider + 'static) {
        *self.0.write().unwrap() = Box::new(text_provider);
    }
}

impl TextProvider for SharedTextProvider {
    fn accept_line_hints(&mut self, line_ids: &[LineId]) {
        self.0.write().unwrap().accept_line_hints(line_ids);
    }

    fn get_text(&self, id: &LineId) -> Option<String> {
        self.0.read().unwrap().get_text(id)
    }

    fn set_language(&mut self, language: Option<Language>) {
        self.0.write().unwrap().set_language(language);
    }

    fn get_language(&self) -> Option<Language> {
        self.0.read().unwrap().get_language()
    }

    fn are_lines_available(&self) -> bool {
        self.0.read().unwrap().are_lines_available()
    }
}
