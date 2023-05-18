use crate::prelude::*;
use bevy::asset::LoadedAsset;
use bevy::prelude::*;
use bevy::{
    asset::{AssetLoader, LoadContext},
    utils::BoxedFuture,
};

pub(crate) fn yarn_slinger_asset_loader_plugin(app: &mut App) {
    app.add_asset::<YarnFile>()
        .init_asset_loader::<YarnFileAssetLoader>();
}

#[derive(Debug, Default)]
pub(crate) struct YarnFileAssetLoader;

impl AssetLoader for YarnFileAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, SystemResult> {
        Box::pin(async move {
            let yarn_file = read_yarn_file(bytes, load_context)?;
            load_context.set_default_asset(LoadedAsset::new(yarn_file));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["yarn"]
    }
}

fn read_yarn_file<'a>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
) -> Result<YarnFile, Error> {
    let source = String::from_utf8(bytes.to_vec())?;
    let file_name = load_context
        .path()
        .file_name()
        .context("Yarn file has no filename")?
        .to_str()
        .context("Yarn file name is not valid UTF-8")?
        .to_owned();
    Ok(YarnFile { file_name, source })
}
