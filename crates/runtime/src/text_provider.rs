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
    fn accept_line_hints(&mut self, line_ids: &[LineId]);
    fn get_text(&self, id: &LineId) -> Option<String>;
    fn set_language(&mut self, language: Option<Language>);
    fn get_language(&self) -> Option<Language>;
    fn are_lines_available(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("The language {language_code:?} is not supported by this text provider")]
pub struct UnsupportedLanguageError {
    language_code: Language,
}

pub type StringTable = HashMap<LineId, String>;

/// A basic implementation of [`TextProvider`] that uses a [`StringTable`] to store the text.
#[derive(Debug, Clone, Default)]
pub struct StringTableTextProvider {
    base_language_table: StringTable,
    translation_table: Option<(Language, StringTable)>,
    language: Option<Language>,
}

impl StringTableTextProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extend_base_language(&mut self, string_table: HashMap<LineId, String>) {
        self.base_language_table.extend(string_table);
    }

    pub fn extend_translation(
        &mut self,
        language: impl Into<Language>,
        string_table: HashMap<LineId, String>,
    ) {
        let language = language.into();
        if let Some((current_language, translation_table)) = self.translation_table.as_mut() {
            if language == *current_language {
                translation_table.extend(string_table);
                return;
            }
        }
        self.translation_table.replace((language, string_table));
    }
}

impl TextProvider for StringTableTextProvider {
    fn accept_line_hints(&mut self, _line_ids: &[LineId]) {
        // no-op
    }

    fn get_text(&self, id: &LineId) -> Option<String> {
        if let Some(language) = self.language.as_ref() {
            if let Some((registered_language, translation_table)) = self.translation_table.as_ref()
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
        self.base_language_table.get(id).cloned()
    }

    fn set_language(&mut self, language_code: Option<Language>) {
        self.language = language_code;
    }

    fn get_language(&self) -> Option<Language> {
        self.language.clone()
    }

    fn are_lines_available(&self) -> bool {
        let Some(language) = self.language.as_ref() else {
            return !self.base_language_table.is_empty();
        };
        let translation_language = self
            .translation_table
            .as_ref()
            .map(|(language, _)| language);
        translation_language == Some(language)
    }
}

#[derive(Debug, Clone)]
pub struct SharedTextProvider(pub Arc<RwLock<dyn TextProvider>>);

impl SharedTextProvider {
    pub fn new(text_provider: impl TextProvider + 'static) -> Self {
        Self(Arc::new(RwLock::new(text_provider)))
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
