//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
use crate::prelude::Language;
use log::error;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use yarn_slinger_core::prelude::*;

/// A trait for providing text to a [`Dialogue`](crate::prelude::Dialogue).
///
/// ## Implementation notes
///
/// By injecting this, we don't need to expose `Dialogue.ExpandSubstitutions` and `Dialogue.ParseMarkup`, since we can apply them internally.
pub trait TextProvider: Debug + Send + Sync {
    fn clone_shallow(&self) -> Box<dyn TextProvider>;
    fn get_text(&self, id: &LineId) -> Option<String>;
    fn set_language_code(
        &mut self,
        language_code: Language,
    ) -> Result<(), UnsupportedLanguageError>;
    fn get_language_code(&self) -> Option<Language>;
}

impl Clone for Box<dyn TextProvider> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("The language {language_code:?} is not supported by this text provider")]
pub struct UnsupportedLanguageError {
    language_code: Language,
}

/// A basic implementation of [`TextProvider`] that uses a [`HashMap`] to store the text.
#[derive(Debug, Clone, Default)]
pub struct StringTableTextProvider {
    base_language_table: Arc<RwLock<HashMap<LineId, String>>>,
    translation_table: Arc<RwLock<HashMap<Language, HashMap<LineId, String>>>>,
    language_code: Arc<RwLock<Option<Language>>>,
}

impl StringTableTextProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_base_language(
        &mut self,
        string_table: HashMap<LineId, String>,
    ) -> HashMap<LineId, String> {
        let mut base_language_table = self.base_language_table.write().unwrap();
        std::mem::replace(&mut *base_language_table, string_table)
    }

    pub fn add_translation(
        &mut self,
        language: impl Into<Language>,
        string_table: HashMap<LineId, String>,
    ) -> Option<HashMap<LineId, String>> {
        self.translation_table
            .write()
            .unwrap()
            .insert(language.into(), string_table)
    }

    pub fn remove_translations_outside_current_language(&mut self) {
        let language_code = self.language_code.read().unwrap();
        if let Some(language_code) = language_code.as_ref() {
            let mut translation_table = self.translation_table.write().unwrap();
            translation_table.retain(|key, _| key == language_code);
        }
    }
}

impl TextProvider for StringTableTextProvider {
    fn clone_shallow(&self) -> Box<dyn TextProvider> {
        Box::new(self.clone())
    }

    fn get_text(&self, id: &LineId) -> Option<String> {
        let language_code = self.language_code.read().unwrap();
        if let Some(language_code) = language_code.as_ref() {
            if let Some(line) = self
                .translation_table
                .read()
                .unwrap()
                .get(language_code)
                .and_then(|table| table.get(id))
            {
                Some(line.clone())
            } else {
                error!("No translation found for line {:?} in language {:?}, falling back to base language.", id, language_code);
                self.base_language_table.read().unwrap().get(id).cloned()
            }
        } else {
            self.base_language_table.read().unwrap().get(id).cloned()
        }
    }

    fn set_language_code(
        &mut self,
        language_code: Language,
    ) -> Result<(), UnsupportedLanguageError> {
        if !self
            .translation_table
            .read()
            .unwrap()
            .contains_key(&language_code)
        {
            return Err(UnsupportedLanguageError { language_code });
        }
        self.language_code.write().unwrap().replace(language_code);
        Ok(())
    }

    fn get_language_code(&self) -> Option<Language> {
        self.language_code.read().unwrap().clone()
    }
}
