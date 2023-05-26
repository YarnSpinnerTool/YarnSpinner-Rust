use crate::prelude::*;
use crate::{GenericAsset, UnderlyingTextProvider};
use bevy::prelude::*;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) fn shared_text_provider_plugin(_app: &mut App) {}

#[derive(Debug, Clone)]
pub(crate) struct SharedTextProvider(Arc<RwLock<Box<dyn TextProvider>>>);

impl SharedTextProvider {
    pub fn new(text_provider: impl TextProvider + 'static) -> Self {
        Self(Arc::new(RwLock::new(Box::new(text_provider))))
    }
    pub fn replace(&mut self, text_provider: impl TextProvider + 'static) {
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

    fn accept_fetched_assets(&mut self, asset: Box<dyn Any>) {
        self.0.write().unwrap().accept_fetched_assets(asset)
    }

    fn fetch_assets(&self) -> Box<dyn Fn(&World) -> GenericAsset + '_> {
        let clone = self.clone();
        Box::new(move |world| clone.0.read().unwrap().fetch_assets()(world))
    }
}

impl UnderlyingTextProvider for SharedTextProvider {
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
}
