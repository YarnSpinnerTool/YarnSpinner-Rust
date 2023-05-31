use crate::default_impl::file_extensions;
use crate::prelude::*;
use bevy::prelude::*;
use std::any::Any;
use std::fmt::Debug;

pub(crate) fn audio_asset_provider_plugin(_app: &mut App) {}

#[derive(Debug, Clone)]
pub struct AudioAssetProvider(FileExtensionAssetProvider);

impl AudioAssetProvider {
    pub fn new() -> Self {
        Self(
            FileExtensionAssetProvider::new().with_file_extensions(file_extensions! {
                AudioSource: ["mp3", "ogg", "wav"],
            }),
        )
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
