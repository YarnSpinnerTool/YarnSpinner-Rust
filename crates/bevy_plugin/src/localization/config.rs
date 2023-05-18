use crate::prelude::*;
use bevy::prelude::*;
use std::path::PathBuf;

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
    pub base: Localization,
    pub others: Vec<Localization>,
    pub file_generation_mode: FileGenerationMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Debug, Default, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Localization {
    /// IETF BCP 47 code. The default is "en-US".
    pub language: String,
    pub strings_file: PathBuf,
    pub assets_sub_folder: PathBuf,
}

impl Default for Localization {
    fn default() -> Self {
        Self::with_language("en-US")
    }
}

impl Localization {
    pub fn with_language(language: impl Into<String>) -> Self {
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
    Debug, Clone, PartialEq, Eq, Hash, Default, Reflect, FromReflect, Serialize, Deserialize,
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
