//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
use crate::prelude::*;
use bevy_platform::collections::HashMap;
use core::any::Any;
use core::fmt::Debug;
use log::error;

/// A trait for providing text to a [`Dialogue`](crate::prelude::Dialogue). The default implementation is [`StringTableTextProvider`], which keeps the
/// text for the base language, i.e. the language the Yarn files are written in, and the text for the currently selected translation in memory.
///
/// ## Implementation notes
///
/// By injecting this, we don't need to expose `Dialogue.ExpandSubstitutions` and `Dialogue.ParseMarkup`, since we can apply them internally.
pub trait TextProvider: Debug + Send + Sync {
    /// Creates a shallow clone of this text provider, i.e. a clone that
    /// shares the same underlying provider and will thus be perfectly in sync
    /// with the original instance.
    fn clone_shallow(&self) -> Box<dyn TextProvider>;
    /// Passes the [`LineId`]s that this [`TextProvider`] should soon provide text for. These are the [`LineId`]s that are contained in the current node and are not required to be actually reached.
    fn accept_line_hints(&mut self, line_ids: &[LineId]);
    /// Returns the text for the given [`LineId`]. Will only be called if [`TextProvider::are_lines_available`] returns `true`.
    fn get_text(&self, id: &LineId) -> Option<String>;
    /// Sets the current language. If `None` is passed, the base language will be used.
    fn set_language(&mut self, language: Option<Language>);
    /// Returns the current language. If `None` is returned, the base language is used.
    fn get_language(&self) -> Option<Language>;
    /// Returns whether the text for all lines announced by [`TextProvider::accept_line_hints`] are available, i.e. have been loaded and are ready to be used.
    fn are_lines_available(&self) -> bool;
    /// Gets the [`TextProvider`] as a trait object.
    /// This allows retrieving the concrete type by downcasting, using the `downcast_ref` method available through the `Any` trait.
    fn as_any(&self) -> &dyn Any;
    /// Gets the [`TextProvider`] as a mutable trait object.
    /// This allows retrieving the concrete type by downcasting, using the `downcast_mut` method available through the `Any` trait.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Clone for Box<dyn TextProvider> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

#[allow(missing_docs)]
pub type StringTable = HashMap<LineId, String>;

/// A basic implementation of [`TextProvider`] which keeps the text for the base language,
/// i.e. the language the Yarn files are written in, and the text for the currently selected translation in memory.
#[derive(Debug, Clone, Default)]
pub struct StringTableTextProvider {
    base_language_table: StringTable,
    translation_table: Option<(Language, StringTable)>,
    /// Set to `None` to select base language.
    translation_language: Option<Language>,
}

impl StringTableTextProvider {
    /// Creates a new [`StringTableTextProvider`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds strings for the base language, i.e. the language that the Yarn files are written in.
    pub fn extend_base_language<T>(&mut self, string_table: impl IntoIterator<Item = T>)
    where
        StringTable: Extend<T>,
    {
        self.base_language_table.extend(string_table);
    }

    /// Adds strings for the a specific language. If this is not the language used selected by [`TextProvider::set_language`], the strings will be ignored.
    pub fn extend_translation<T>(
        &mut self,
        language: impl Into<Language>,
        string_table: impl IntoIterator<Item = T>,
    ) where
        StringTable: Extend<T>,
    {
        let language = language.into();
        if let Some((current_language, translation_table)) = self.translation_table.as_mut()
            && language == *current_language
        {
            translation_table.extend(string_table);
            return;
        }

        let (language, mut table) = self
            .translation_table
            .take()
            .unwrap_or_else(|| (language, StringTable::new()));

        table.clear();
        table.extend(string_table);

        self.translation_table = Some((language, table));
    }
}

impl TextProvider for StringTableTextProvider {
    fn clone_shallow(&self) -> Box<dyn TextProvider> {
        Box::new(self.clone())
    }

    fn accept_line_hints(&mut self, _line_ids: &[LineId]) {
        // no-op
    }

    fn get_text(&self, id: &LineId) -> Option<String> {
        if let Some(language) = self.translation_language.as_ref()
            && let Some((registered_language, translation_table)) = self.translation_table.as_ref()
        {
            if registered_language != language {
                error!(
                    "Didn't find language {language} in translations, falling back to base language."
                );
            } else if let Some(line) = translation_table.get(id) {
                return Some(line.clone());
            } else {
                error!(
                    "No translation found for line {id} in language {language}, falling back to base language."
                );
            }
        }
        self.base_language_table.get(id).cloned()
    }

    fn set_language(&mut self, language_code: Option<Language>) {
        self.translation_language = language_code;
    }

    fn get_language(&self) -> Option<Language> {
        self.translation_language.clone()
    }

    fn are_lines_available(&self) -> bool {
        let Some(language) = self.translation_language.as_ref() else {
            return !self.base_language_table.is_empty();
        };
        let translation_language = self
            .translation_table
            .as_ref()
            .map(|(language, _)| language);
        translation_language == Some(language)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
