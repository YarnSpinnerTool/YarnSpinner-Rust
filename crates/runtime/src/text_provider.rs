//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
use crate::prelude::Language;
use log::error;
use std::any::Any;
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
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn accept_line_hints(&self, line_ids: &[LineId]);
    fn get_text(&self, id: &LineId) -> Option<String>;
    fn set_language(&mut self, language: Option<Language>);
    fn get_language(&self) -> Option<Language>;
    fn lines_available(&self) -> bool;
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

pub type StringTable = HashMap<LineId, String>;

/// A basic implementation of [`TextProvider`] that uses a [`HashMap`] to store the text.
#[derive(Debug, Clone, Default)]
pub struct StringTableTextProvider {
    base_language_table: Arc<RwLock<StringTable>>,
    translation_table: Arc<RwLock<Option<(Language, StringTable)>>>,
    language: Arc<RwLock<Option<Language>>>,
}

impl StringTableTextProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extend_base_language(&mut self, string_table: HashMap<LineId, String>) {
        let mut base_language_table = self.base_language_table.write().unwrap();
        base_language_table.extend(string_table);
    }

    pub fn extend_translation(
        &mut self,
        language: impl Into<Language>,
        string_table: HashMap<LineId, String>,
    ) {
        let language = language.into();
        let mut translation_table = self.translation_table.write().unwrap();
        if let Some((current_language, translation_table)) = translation_table.as_mut() {
            if language == *current_language {
                translation_table.extend(string_table);
                return;
            }
        }
        translation_table.replace((language, string_table));
    }
}

impl TextProvider for StringTableTextProvider {
    fn clone_shallow(&self) -> Box<dyn TextProvider> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn accept_line_hints(&self, _line_ids: &[LineId]) {
        // no-op
    }

    fn get_text(&self, id: &LineId) -> Option<String> {
        let language = self.language.read().unwrap();
        if let Some(language) = language.as_ref() {
            if let Some((registered_language, translation_table)) =
                self.translation_table.read().unwrap().as_ref()
            {
                if registered_language != language {
                    error!("Didn't find language {language} in translations, falling back to base language.");
                } else if let Some(line) = translation_table.get(id) {
                    return Some(line.clone());
                } else {
                    error!("No translation found for line {id} in language {language}, falling back to base language.");
                }
            }
        }
        self.base_language_table.read().unwrap().get(id).cloned()
    }

    fn set_language(&mut self, language_code: Option<Language>) {
        *self.language.write().unwrap() = language_code;
    }

    fn get_language(&self) -> Option<Language> {
        self.language.read().unwrap().clone()
    }

    fn lines_available(&self) -> bool {
        let language = self.language.read().unwrap();
        let Some(language) = language.as_ref() else {
            return !self.base_language_table.read().unwrap().is_empty();
        };
        let translation_table = self.translation_table.read().unwrap();
        let translation_language = translation_table.as_ref().map(|(language, _)| language);
        translation_language == Some(language)
    }
}
