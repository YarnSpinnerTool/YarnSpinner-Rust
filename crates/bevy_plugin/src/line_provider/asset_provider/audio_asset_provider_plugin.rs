use crate::default_impl::file_extensions;
use crate::prelude::*;
use bevy::prelude::*;
use std::any::Any;
use std::fmt::Debug;

pub(crate) fn audio_asset_provider_plugin(_app: &mut App) {}

/// A wrapper around [`FileExtensionAssetProvider`] that is configured to load audio assets.
/// See [`FileExtensionAssetProvider`] for information on how assets are searched.
///
/// Because this asset provider requires knowledge of the current language, it will only fetch assets if you set up Yarn Slinger with [`Localizations`] using
/// [`YarnSlingerPlugin::with_localizations`] or [`LoadYarnProjectEvent::with_localizations`](crate::deferred_loading::LoadYarnProjectEvent::with_localizations).
///
/// Requires the `audio_assets` feature, in which case it can be used in a [`DialogueRunner`] by calling [`DialogueRunnerBuilder::add_asset_provider`].
#[derive(Debug, Clone)]
pub struct AudioAssetProvider(FileExtensionAssetProvider);

impl Default for AudioAssetProvider {
    fn default() -> Self {
        Self(
            FileExtensionAssetProvider::new().with_file_extensions(file_extensions! {
                AudioSource: ["mp3", "ogg", "wav"],
            }),
        )
    }
}

impl AudioAssetProvider {
    /// Initializes a new [`AudioAssetProvider`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl AssetProvider for AudioAssetProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_language(&self) -> Option<Language> {
        self.0.get_language()
    }

    fn set_language(&mut self, language: Option<Language>) {
        self.0.set_language(language)
    }

    fn set_localizations(&mut self, localizations: Localizations) {
        self.0.set_localizations(localizations)
    }

    fn set_asset_server(&mut self, asset_server: AssetServer) {
        self.0.set_asset_server(asset_server)
    }

    fn are_assets_available(&self) -> bool {
        self.0.are_assets_available()
    }

    fn accept_line_hints(&mut self, line_ids: &[LineId]) {
        self.0.accept_line_hints(line_ids)
    }

    fn get_assets(&self, line: &YarnLine) -> LineAssets {
        self.0.get_assets(line)
    }
}
