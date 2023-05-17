use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration for localization strategy used by the [`YarnSlingerPlugin`](crate::prelude::YarnSlingerPlugin).
/// Its default variant, which is [`YarnSlingerLocalizationConfig::noninteractive`], is inserted when the plugin is added.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect, FromReflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Resource, PartialEq, Default)]
#[cfg_attr(feature = "serde", reflect(Serialize, Deserialize))]
pub struct YarnSlingerLocalizationConfig {
    /// If active will ensure that a loaded yarn file has line IDs for all nodes by generating them and replacing the file on disk.
    /// Line IDs are necessary to generate localization files, as only the line ID is used to identify a line in a localization file.
    pub generate_missing_line_ids_in_yarn_file: bool,
    /// If active will ensure that loaded localization files have translations for all line IDs.
    /// It does so by looking at which languages are present in the file and then panicking if any line does not support all languages.
    /// Note that if you combine this with [`YarnSlingerDefaultLocaleConfig::append_missing_line_ids_to_localization_files`],
    /// you will instantly panic if a line ID is added at runtime.
    pub panic_on_missing_localization_when_loading_assets: bool,
    /// Settings that only make sense if you have specified a default locale, i.e. the language you are writing your `.yarn` files in.
    /// Note that this is only the language your raw Yarn files are written in, and *not* the locale your game is running in right now.
    pub default_locale_config: Option<YarnSlingerDefaultLocaleConfig>,
}

impl Default for YarnSlingerLocalizationConfig {
    fn default() -> Self {
        Self::noninteractive()
    }
}

/// Additions to [`YarnSlingerLocalizationConfig`] that require you to specify a default locale, i.e. the language you are writing your `.yarn` files in.
/// Note that this is only the language your raw Yarn files are written in, and *not* the locale your game is running in right now.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect, FromReflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(PartialEq)]
#[cfg_attr(feature = "serde", reflect(Serialize, Deserialize))]
pub struct YarnSlingerDefaultLocaleConfig {
    /// The default locale that is referenced by the other fields of [`YarnSlingerDefaultLocaleConfig`]
    pub default_locale: String,
    /// If active will generate a localization file on disk for a loaded yarn file if it does not have one already.
    /// The generated file will contain all lines with IDs set to the [`YarnSlingerDefaultLocaleConfig::default_locale`].
    pub generate_missing_localization_files: bool,
    /// If active will append missing line IDs to a localization file on disk for a loaded yarn file.
    /// If there is no localization file and [`YarnSlingerDefaultLocaleConfig::generate_missing_localization_files`] is not active, nothing happens.
    pub append_missing_line_ids_to_localization_files: bool,
    /// If active will remove line IDs from a localization file on disk if they are considered unused. For this to be the case, the following conditions must be met:
    /// - The line ID referenced in the localization file is not present in the yarn file
    /// - The line is only available for the [`YarnSlingerDefaultLocaleConfig::default_locale`]
    pub remove_unused_line_ids_from_localization_files: bool,
    /// If active will fall back to the [`YarnSlingerDefaultLocaleConfig::default_locale`] if a localization is missing for a line while executing the program, emitting a warning.
    /// Note that when [`YarnSlingerLocalizationConfig::panic_on_missing_localization_when_loading_assets`] is active **and**
    /// [`YarnSlingerDefaultLocaleConfig::append_missing_line_ids_to_localization_files`] is not active, this can logically never happen.
    pub fall_back_to_default_locale_on_missing_localization_when_presenting_line: bool,
}

impl YarnSlingerLocalizationConfig {
    /// The default config. Generates neither line IDs nor localization files, but emits errors if you try to load a localization file that is only partially complete.
    /// This setting is useful for two scenarios:
    /// - You want to prototype a game without dealing with localization.
    /// - You are shipping a game that is already localized
    pub fn noninteractive() -> Self {
        Self {
            generate_missing_line_ids_in_yarn_file: false,
            panic_on_missing_localization_when_loading_assets: true,
            default_locale_config: None,
        }
    }

    /// Generates everything you need to setup localization on the fly while the game is running.
    /// If you encounter a missing localization, it will fall back to the default locale.
    /// Use this setting if you are working on your localization and want to see the results immediately or you don't want to crash if you forgot to add a localization.
    pub fn interactive_with_default_locale(locale: impl Into<String>) -> Self {
        Self {
            generate_missing_line_ids_in_yarn_file: true,
            panic_on_missing_localization_when_loading_assets: false,
            default_locale_config: Some(YarnSlingerDefaultLocaleConfig {
                default_locale: locale.into(),
                generate_missing_localization_files: true,
                append_missing_line_ids_to_localization_files: true,
                remove_unused_line_ids_from_localization_files: true,
                fall_back_to_default_locale_on_missing_localization_when_presenting_line: true,
            }),
        }
    }
}
