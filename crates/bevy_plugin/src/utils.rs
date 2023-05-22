use crate::prelude::*;
use crate::project::YarnProjectConfigToLoad;
use bevy::asset::FileAssetIo;
use bevy::prelude::*;
use std::path::Path;

pub(crate) fn panic_on_err(In(result): In<SystemResult>) {
    if let Err(e) = result {
        panic!("Error in Yarn Slinger plugin: {e}");
    }
}

pub(crate) fn in_development(
    project: Option<Res<YarnProject>>,
    project_to_load: Option<Res<YarnProjectConfigToLoad>>,
) -> bool {
    if let Some(project) = project {
        if let Some(localizations) = project.localizations.as_ref() {
            return localizations.file_generation_mode == FileGenerationMode::Development;
        }
    }
    if let Some(project_to_load) = project_to_load {
        if let Some(Some(ref localizations)) = project_to_load.localizations {
            return localizations.file_generation_mode == FileGenerationMode::Development;
        }
    }
    false
}

pub(crate) fn get_assets_dir_path(asset_server: &AssetServer) -> Result<impl AsRef<Path> + '_> {
    let asset_io = asset_server.asset_io();
    let file_asset_io = asset_io.downcast_ref::<FileAssetIo>().context(
        "Failed to downcast asset server IO to `FileAssetIo`. \
    The vanilla Bevy `FileAssetIo` is the only one supported by Yarn Slinger",
    )?;
    Ok(file_asset_io.root_path())
}
