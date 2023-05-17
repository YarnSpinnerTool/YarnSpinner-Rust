use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration for the [`YarnSlingerPlugin`]. Its default variant is inserted when the plugin is added.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect, FromReflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Resource, PartialEq, Default)]
#[cfg_attr(feature = "serde", reflect(Serialize, Deserialize))]
pub struct YarnSlingerConfig {
    pub generate_missing_line_ids_in_yarn_file: bool,
    pub generate_missing_localization_files: bool,
    pub append_missing_line_ids_to_localization_files: bool,
    pub error_on_missing_localization_on_load: bool,
    pub runtime_fallback_locale: Option<String>,
}

impl Default for YarnSlingerConfig {
    fn default() -> Self {
        Self {
            generate_missing_line_ids_in_yarn_file: true,
            generate_missing_localization_files: true,
            append_missing_line_ids_to_localization_files: true,
            error_on_missing_localization_on_load: true,
            runtime_fallback_locale: None,
        }
    }
}
