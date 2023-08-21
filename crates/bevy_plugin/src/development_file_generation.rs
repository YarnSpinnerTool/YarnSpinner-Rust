use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn development_file_generation_plugin(app: &mut App) {
    app.register_type::<DevelopmentFileGeneration>();
}

/// The kind of development experience you wish when creating Yarn files and dealing with missing localizations.
/// Defaults to [`DevelopmentFileGeneration::TRY_FULL`] in debug builds, [`DevelopmentFileGeneration::None`] otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Debug, Default, PartialEq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DevelopmentFileGeneration {
    /// The recommended setting for a development environment:
    /// - Generates line IDs for all lines in loaded Yarn files and writes them back to disk.
    /// - Generates new strings files for all languages that are missing them, filling them with the lines found in the Yarn files.
    /// - Adds new lines to strings files when they have been added to a loaded Yarn file.
    /// - Marks lines in strings files that have been changed since they were translated by appending "NEEDS UPDATE" to the respective line texts.
    ///
    /// It is recommended to combine this setting with Bevy's [hot reload functionality](https://bevy-cheatbook.github.io/assets/hot-reload.html).
    /// Note that because of the extensive use of the filesystem, this setting is not available on Wasm or Android.
    Full,
    /// The recommended setting for shipping the game:
    /// - Does not change any Yarn or strings files on disk.
    /// - Falls back to the base language when a line is missing in a strings file.
    None,
}

impl DevelopmentFileGeneration {
    /// [`DevelopmentFileGeneration::Full`] on all platforms except Wasm and Android, [`DevelopmentFileGeneration::None`] otherwise.
    pub const TRY_FULL: Self = {
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            Self::Full
        }
        #[cfg(any(target_arch = "wasm32", target_os = "android"))]
        {
            Self::None
        }
    };
}

impl Default for DevelopmentFileGeneration {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        {
            Self::TRY_FULL
        }
        #[cfg(not(debug_assertions))]
        {
            Self::None
        }
    }
}
