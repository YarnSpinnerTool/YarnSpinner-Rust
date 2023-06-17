use crate::prelude::*;
use crate::project::DEFAULT_ASSET_DIR;
use bevy::prelude::*;
use std::iter;
use std::path::{Path, PathBuf};

pub(crate) fn localization_config_plugin(_app: &mut App) {}

/// The localizations used by the [`YarnProject`]. Can be set with [`YarnSlingerPlugin::with_localizations`] or
/// [`LoadYarnProjectEvent::with_localizations`](crate::deferred_loading::LoadYarnProjectEvent::with_localizations).
///
/// ## Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_yarn_slinger::prelude::*;
///
/// let localizations = Localizations {
///     base_localization: "en-US".into(),
///     translations: vec!["de-CH".into(), "fr-FR".into()],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Localizations {
    /// The language the Yarn files themselves are written in.
    pub base_localization: Localization,
    /// The supported translations of the Yarn files. Will be loaded from somewhere by the [`TextProvider`].
    /// See [`StringsFileTextProvider`](crate::default_impl::StringsFileTextProvider) for how this is done by default.
    pub translations: Vec<Localization>,
}

impl Localizations {
    /// Returns whether the given language is supported by these [`Localizations`] as either a base language or a translation.
    pub fn supports_language(&self, language: &Language) -> bool {
        self.supported_languages()
            .any(|supported_language| supported_language == language)
    }

    /// Returns the localization for the given translation, if it exists. Will return [`None`] if the given language is not supported or the base language.
    pub(crate) fn translation(&self, language: &Language) -> Option<&Localization> {
        self.translations
            .iter()
            .find(|localization| localization.language == *language)
    }

    pub(crate) fn supported_localization(&self, language: &Language) -> Option<&Localization> {
        iter::once(&self.base_localization)
            .chain(self.translations.iter())
            .find(|localization| localization.language == *language)
    }

    /// Iterates over all supported languages, including the base language.
    pub fn supported_languages(&self) -> impl Iterator<Item = &Language> {
        iter::once(&self.base_localization.language).chain(
            self.translations
                .iter()
                .map(|localization| &localization.language),
        )
    }

    pub(crate) fn strings_file_path(&self, language: impl Into<Language>) -> Option<&Path> {
        let language = language.into();
        self.translations
            .iter()
            .find_map(|t| (t.language == language).then_some(t.strings_file.as_path()))
    }
}

/// A supported localization inside [`Localizations`]. Created with [`Localization::with_language`].
/// You can create this type from types that implement [`Into<Language>`], like this:
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_yarn_slinger::prelude::*;
/// let localization: Localization = "de-CH".into();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Localization {
    /// The language of this localization.
    pub language: Language,
    /// The path to the strings file for this localization inside the `assets` folder.
    /// Defaults to `dialogue/{language}.strings.csv`. So, for the language "de-CH", you'd end up with "assets/dialogue/de-CH.strings.csv".
    pub strings_file: PathBuf,
    /// The path to the subdirectory containing the assets for this localization inside the `assets` folder.
    /// Defaults to `dialogue/{language}/`.  So, for the language "de-CH", you'd end up with "assets/dialogue/de-CH/".
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
    /// Creates a new [`Localization`] with the given language.
    /// Can also be created from types that implement [`Into<Language>`], like this:
    /// ```rust
    /// # use bevy::prelude::*;
    /// # use bevy_yarn_slinger::prelude::*;
    /// let localization: Localization = "de-CH".into();
    /// ```
    pub fn with_language(language: impl Into<Language>) -> Self {
        let language = language.into();
        let strings_file = PathBuf::from(format!("{DEFAULT_ASSET_DIR}/{language}.strings.csv"));
        let assets_sub_folder = PathBuf::from(format!("{DEFAULT_ASSET_DIR}/{language}/"));
        Self {
            language,
            strings_file,
            assets_sub_folder,
        }
    }

    /// Sets the path to the strings file for this localization inside the `assets` folder.
    pub fn with_strings_file(mut self, strings_file: impl Into<PathBuf>) -> Self {
        self.strings_file = strings_file.into();
        self
    }

    /// Sets the path to the subdirectory containing the assets for this localization inside the `assets` folder.
    pub fn with_assets_sub_folder(mut self, assets_sub_folder: impl Into<PathBuf>) -> Self {
        self.assets_sub_folder = assets_sub_folder.into();
        self
    }
}
