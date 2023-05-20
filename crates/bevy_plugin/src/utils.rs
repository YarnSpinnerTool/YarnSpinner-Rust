use crate::prelude::*;
use bevy::asset::FileAssetIo;
use bevy::prelude::*;
use std::path::Path;

pub(crate) fn panic_on_err(In(result): In<SystemResult>) {
    if let Err(e) = result {
        panic!("Error in Yarn Slinger plugin: {e}");
    }
}

pub(crate) fn is_in_development(localizations: Option<Res<Localizations>>) -> bool {
    localizations
        .as_ref()
        .map(|localizations| localizations.file_generation_mode == FileGenerationMode::Development)
        .unwrap_or_default()
}

pub(crate) fn get_assets_dir_path(asset_server: &AssetServer) -> Result<impl AsRef<Path> + '_> {
    let asset_io = asset_server.asset_io();
    let file_asset_io = asset_io.downcast_ref::<FileAssetIo>().context(
        "Failed to downcast asset server IO to `FileAssetIo`. \
    The vanilla Bevy `FileAssetIo` is the only one supported by Yarn Slinger",
    )?;
    Ok(file_asset_io.root_path())
}
