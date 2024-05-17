use crate::prelude::*;
use crate::UnderlyingTextProvider;
use bevy::prelude::*;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) fn shared_text_provider_plugin(_app: &mut App) {}

/// A [`TextProvider`] that wraps another [`TextProvider`] and is shallow cloned. It can thus be shared between users.
#[derive(Debug, Clone)]
pub(crate) struct SharedTextProvider(Arc<RwLock<Box<dyn TextProvider>>>);

impl SharedTextProvider {
    /// Creates a new [`SharedTextProvider`] wrapping a [`TextProvider`].
    pub(crate) fn new(text_provider: impl TextProvider + 'static) -> Self {
        Self(Arc::new(RwLock::new(Box::new(text_provider))))
    }

    /// Replace the underlying [`TextProvider`] with another one. All copies of this [`SharedTextProvider`] will be affected.
    pub(crate) fn replace(&mut self, text_provider: impl TextProvider + 'static) {
        *self.0.write().unwrap() = Box::new(text_provider);
    }
}

impl TextProvider for SharedTextProvider {
    fn set_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>) {
        self.0.write().unwrap().set_base_string_table(string_table)
    }

    fn extend_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>) {
        self.0
            .write()
            .unwrap()
            .extend_base_string_table(string_table)
    }

    fn take_fetched_assets(&mut self, asset: Box<dyn Any>) {
        self.0.write().unwrap().take_fetched_assets(asset)
    }

    fn fetch_assets(&self, world: &World) -> Option<Box<dyn Any + 'static>> {
        self.0.read().unwrap().fetch_assets(world)
    }
}

impl UnderlyingTextProvider for SharedTextProvider {
    fn clone_shallow(&self) -> Box<dyn UnderlyingTextProvider> {
        Box::new(self.clone())
    }

    fn accept_line_hints(&mut self, line_ids: &[LineId]) {
        self.0.write().unwrap().accept_line_hints(line_ids)
    }

    fn get_text(&self, id: &LineId) -> Option<String> {
        self.0.read().unwrap().get_text(id)
    }

    fn set_language(&mut self, language: Option<Language>) {
        self.0.write().unwrap().set_language(language)
    }

    fn get_language(&self) -> Option<Language> {
        self.0.read().unwrap().get_language()
    }

    fn are_lines_available(&self) -> bool {
        self.0.read().unwrap().are_lines_available()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
