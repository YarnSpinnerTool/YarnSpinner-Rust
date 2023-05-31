use crate::prelude::*;
use crate::UnderlyingTextProvider;
use bevy::asset::LoadState;
use bevy::prelude::*;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

pub(crate) fn strings_file_text_provider_plugin(_app: &mut App) {}

#[derive(Clone)]
pub struct StringsFileTextProvider {
    asset_server: AssetServer,
    localizations: Option<Localizations>,
    language: Option<Language>,
    base_string_table: HashMap<LineId, StringInfo>,
    strings_file_handle: Option<Handle<StringsFile>>,
    translation_string_table: Option<HashMap<LineId, String>>,
}

impl Debug for StringsFileTextProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringsTableTextProvider")
            .field("asset_server", &())
            .field("localizations", &self.localizations)
            .finish()
    }
}

impl UnderlyingTextProvider for StringsFileTextProvider {
    fn accept_line_hints(&mut self, _line_ids: &[LineId]) {
        // no-op
    }

    fn get_text(&self, id: &LineId) -> Option<String> {
        self.translation_string_table
            .as_ref()
            .and_then(|table| table.get(id).cloned())
            .or_else(|| {
                if let Some(language) = self.language.as_ref() {
                    if self.translation_string_table.is_some() {
                        warn!("Did not find translation for line {id} in language {language} because it is untranslated, falling back to base language.");
                    } else {
                        warn!("Did not find translation for line {id} in language {language} because the strings file has not been loaded yet, falling back to base language.");
                    }
                }
                self.base_string_table.get(id).map(|info| info.text.clone())
            })
    }

    fn set_language(&mut self, language: Option<Language>) {
        if language == self.language {
            return;
        }

        self.set_language_invalidating_translation(language.clone());
        let Some(language) = language else {
            return;
        };

        let Some(localizations) = self.localizations.as_ref() else {
            panic!("Set language to {language}, but no localizations have been registered as supported.");
        };
        if language == localizations.base_language.language {
            self.set_language_invalidating_translation(None);
            return;
        }
        let Some(localization) = localizations.translation(&language) else {
            let languages = localizations.supported_languages().map(|l| l.as_ref()).collect::<Vec<_>>().join(", ");
            panic!("Set language to {language}, but that language is not supported. Expected one of {languages}.");
        };
        let path = localization.strings_file.as_path();
        if self.asset_server.asset_io().is_file(path) {
            self.strings_file_handle
                .replace(self.asset_server.load(path));
        } else {
            panic!("Set language to {language}, but the expected strings file at {path} does not exist.", path = path.display());
        }
    }

    fn get_language(&self) -> Option<Language> {
        self.language.clone()
    }

    fn are_lines_available(&self) -> bool {
        let is_base_language = self.language.is_none();
        let has_fetched_translation = || self.translation_string_table.is_some();
        is_base_language || has_fetched_translation()
    }
}

impl StringsFileTextProvider {
    pub fn from_yarn_project(yarn_project: &YarnProject) -> Self {
        Self {
            asset_server: yarn_project.asset_server.clone(),
            localizations: yarn_project.localizations.clone(),
            language: None,
            base_string_table: yarn_project.compilation.string_table.clone(),
            strings_file_handle: None,
            translation_string_table: None,
        }
    }
    fn set_language_invalidating_translation(&mut self, language: impl Into<Option<Language>>) {
        self.language = language.into();
        self.translation_string_table = None;
        self.strings_file_handle = None;
    }
}

impl TextProvider for StringsFileTextProvider {
    fn set_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>) {
        self.base_string_table = string_table;
    }

    fn extend_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>) {
        self.base_string_table.extend(string_table);
    }

    fn take_fetched_assets(&mut self, asset: Box<dyn Any>) {
        let string_table: Box<HashMap<LineId, String>> = asset.downcast().unwrap();
        self.translation_string_table.replace(*string_table);
    }

    fn fetch_assets(&self, world: &World) -> Option<Box<dyn Any + 'static>> {
        let Some(handle) = self.strings_file_handle.as_ref() else {
            return None;
        };
        if self.asset_server.get_load_state(handle) != LoadState::Loaded {
            return None;
        }
        let asset_events = world.resource::<Events<AssetEvent<StringsFile>>>();
        let strings_file_has_changed = || {
            asset_events
                .iter_current_update_events()
                .filter_map(|event| {
                    if let AssetEvent::Modified { handle } = event {
                        Some(handle)
                    } else {
                        None
                    }
                })
                .any(|h| h == handle)
        };
        let has_no_translation_yet = self.translation_string_table.is_none();
        if has_no_translation_yet || strings_file_has_changed() {
            let strings_file = world.resource::<Assets<StringsFile>>().get(handle).unwrap();
            let expected_language = self.language.as_ref().unwrap();
            if let Some(record) = strings_file.get_offending_language(expected_language) {
                let path = self.asset_server.get_handle_path(handle).unwrap();
                panic!("Expected strings file at {path} to only contain language {expected_language}, but its entry with id \"{id}\" is for language {actual_language}.",
                           path = path.path().display(),
                           id = record.id,
                           actual_language = record.language,
                    );
            }
            let string_table: HashMap<LineId, String> = strings_file
                .get_texts()
                .map(|(id, record)| (id.clone(), record.clone()))
                .collect();
            Some(Box::new(string_table))
        } else {
            None
        }
    }
}
