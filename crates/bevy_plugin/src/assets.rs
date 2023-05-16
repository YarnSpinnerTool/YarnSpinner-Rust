use anyhow::Context;
use bevy::asset::LoadedAsset;
use bevy::{
    asset::{AssetLoader, Error, LoadContext},
    utils::BoxedFuture,
};
use yarn_slinger::prelude::*;

#[derive(Default)]
pub struct CustomAssetLoader;

impl AssetLoader for CustomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let source = String::from_utf8(bytes.to_vec())?;
            let file_name = load_context
                .path()
                .file_name()
                .context("Yarn file has no filename")?
                .to_str()
                .context("Yarn file name is not valid UTF-8")?
                .to_owned();
            let yarn_file = YarnFile { file_name, source };
            load_context.set_default_asset(LoadedAsset::new(yarn_file));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["yarn"]
    }
}
