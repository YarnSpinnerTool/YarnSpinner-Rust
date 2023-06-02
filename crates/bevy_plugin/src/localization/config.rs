use crate::prelude::*;
use bevy::prelude::*;
use std::iter;
use std::path::{Path, PathBuf};

pub(crate) fn localization_config_plugin(app: &mut App) {
    app.register_type::<Localizations>()
        .register_type::<Localization>()
        .register_type::<FileGenerationMode>();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct Localizations {
    pub base_language: Localization,
    pub translations: Vec<Localization>,
    pub file_generation_mode: FileGenerationMode,
}

impl Localizations {
    pub fn supports_language(&self, language: impl AsRef<str>) -> bool {
        self.supported_languages()
            .any(|supported_language| supported_language.as_ref() == language.as_ref())
    }

    pub(crate) fn translation(&self, language: impl AsRef<str>) -> Option<&Localization> {
        let language = language.as_ref();
        self.translations
            .iter()
            .find(|localization| localization.language.as_ref() == language)
    }

    pub(crate) fn supported_localization(
        &self,
        language: impl AsRef<str>,
    ) -> Option<&Localization> {
        let language = language.as_ref();
        iter::once(&self.base_language)
            .chain(self.translations.iter())
            .find(|localization| localization.language.as_ref() == language)
    }

    pub fn supported_languages(&self) -> impl Iterator<Item = &Language> {
        iter::once(&self.base_language.language).chain(
            self.translations
                .iter()
                .map(|localization| &localization.language),
        )
    }

    pub(crate) fn strings_file_path(&self, language: impl AsRef<str>) -> Option<&Path> {
        let language = language.as_ref();
        self.translations
            .iter()
            .find_map(|t| (t.language.as_ref() == language).then_some(t.strings_file.as_path()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct Localization {
    pub language: Language,
    pub strings_file: PathBuf,
    pub assets_sub_folder: PathBuf,
}

impl<T> From<T> for Localization
where
    Language: From<T>,
{
    fn from(language: T) -> Self {
        Self::with_language(language)
    }
}

impl Localization {
    pub fn with_language(language: impl Into<Language>) -> Self {
        let language = language.into();
        let strings_file = PathBuf::from(format!("{language}.strings.csv"));
        let assets_sub_folder = PathBuf::from(format!("{language}/"));
        Self {
            language,
            strings_file,
            assets_sub_folder,
        }
    }

    pub fn with_strings_file(mut self, strings_file: impl Into<PathBuf>) -> Self {
        self.strings_file = strings_file.into();
        self
    }

    pub fn with_assets_sub_folder(mut self, assets_sub_folder: impl Into<PathBuf>) -> Self {
        self.assets_sub_folder = assets_sub_folder.into();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Debug, Default, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FileGenerationMode {
    /// The recommended setting for a development environment:
    /// - Generates line IDs for all lines in loaded yarn files and writes them back to disk.
    /// - Generates new strings files for all languages that are missing them, filling them with the lines found in the Yarn files.
    /// - Adds new lines to strings files when they have been added to a loaded Yarn file.
    /// - Marks lines in strings files that have been changed since they were translated by appending "NEEDS UPDATE" to the respective line texts.
    ///
    /// It is recommended to combine this setting with Bevy's [hot reload functionality](https://bevy-cheatbook.github.io/assets/hot-reload.html).
    /// Note that because of the extensive use of the filesystem, this setting is not available on Wasm.
    Development,
    /// The recommended setting for shipping the game:
    /// - Does not change any Yarn or strings files on disk.
    /// - Falls back to the base language when a line is missing in a strings file.
    Production,
}

impl Default for FileGenerationMode {
    fn default() -> Self {
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            Self::Development
        }
        #[cfg(any(target_arch = "wasm32", target_os = "android"))]
        {
            Self::Production
        }
    }
}
