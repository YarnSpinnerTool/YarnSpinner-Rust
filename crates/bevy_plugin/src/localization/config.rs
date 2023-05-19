use crate::prelude::*;
use bevy::prelude::*;
use std::path::{Path, PathBuf};

pub(crate) fn localization_config_plugin(app: &mut App) {
    app.register_type::<Localizations>()
        .register_type::<Localization>()
        .register_type::<FileGenerationMode>();
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    Hash,
    Resource,
    Reflect,
    FromReflect,
    Serialize,
    Deserialize,
)]
#[reflect(Debug, Resource, Default, PartialEq, Hash, Serialize, Deserialize)]
pub struct Localizations {
    pub base_language: Localization,
    pub translations: Vec<Localization>,
    pub file_generation_mode: FileGenerationMode,
}

impl Localizations {
    pub fn supports_translation(&self, language: impl Into<Language>) -> bool {
        let language = language.into();
        self.translations
            .iter()
            .any(|localization| localization.language == language)
    }

    pub(crate) fn get_strings_file(&self, language: impl Into<Language>) -> Option<&Path> {
        let language = language.into();
        self.translations
            .iter()
            .find_map(|t| (t.language == language).then_some(t.strings_file.as_path()))
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Debug, Default, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Localization {
    /// The default is "en-US".
    pub language: Language,
    pub strings_file: PathBuf,
    pub assets_sub_folder: PathBuf,
}

impl From<String> for Localization {
    fn from(language: String) -> Self {
        Self::with_language(language)
    }
}

impl From<&str> for Localization {
    fn from(language: &str) -> Self {
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
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
    #[default]
    Development,
    /// The recommended setting for shipping the game:
    /// - Does not change any Yarn or strings files on disk.
    /// - Falls back to the base language when a line is missing in a strings file.
    Production,
}
