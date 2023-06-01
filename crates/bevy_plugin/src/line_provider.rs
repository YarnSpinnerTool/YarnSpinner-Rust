use crate::prelude::*;
#[cfg(feature = "audio_assets")]
pub use asset_provider::AudioAssetProvider;
pub use asset_provider::{file_extensions, AssetProvider, FileExtensionAssetProvider, LineAssets};
use bevy::prelude::*;
pub(crate) use text_provider::SharedTextProvider;
pub use text_provider::{StringsFileTextProvider, TextProvider};

mod asset_provider;
mod text_provider;

pub(crate) fn line_provider_plugin(app: &mut App) {
    app.fn_plugin(asset_provider::asset_provider_plugin)
        .fn_plugin(text_provider::text_provider_plugin);
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, SystemSet)]
pub(crate) struct LineProviderSystemSet;
