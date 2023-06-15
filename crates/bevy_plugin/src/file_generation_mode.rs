use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn file_generation_mode_plugin(app: &mut App) {
    app.register_type::<FileGenerationMode>();
}

/// The kind of development experience you wish when creating yarn files and dealing with missing localizations.
/// Defaults to [`FileGenerationMode::DEVELOPMENT_ON_SUPPORTED_PLATFORMS`] in debug builds, [`FileGenerationMode::Production`] otherwise.
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

impl FileGenerationMode {
    /// [`FileGenerationMode::Development`] on all platforms except Wasm and Android, [`FileGenerationMode::Production`] otherwise.
    pub const DEVELOPMENT_ON_SUPPORTED_PLATFORMS: Self = {
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            Self::Development
        }
        #[cfg(any(target_arch = "wasm32", target_os = "android"))]
        {
            Self::Production
        }
    };
}

impl Default for FileGenerationMode {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        {
            Self::DEVELOPMENT_ON_SUPPORTED_PLATFORMS
        }
        #[cfg(not(debug_assertions))]
        {
            Self::Production
        }
    }
}
