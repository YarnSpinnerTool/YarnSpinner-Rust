use crate::config::YarnSlingerConfig;
use anyhow::Context;
use bevy::asset::LoadedAsset;
use bevy::{
    asset::{AssetLoader, Error, LoadContext},
    utils::BoxedFuture,
};
use yarn_slinger::prelude::*;

#[derive(Debug, Default)]
pub struct YarnFileAssetLoader {
    config: YarnSlingerConfig,
}

impl AssetLoader for YarnFileAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let yarn_file = read_yarn_file(bytes, load_context)?;
            if self.config.append_missing_line_ids_to_localization_files {
                todo!()
            }
            if self.config.generate_missing_localization_files {
                todo!()
            }
            if self.config.append_missing_line_ids_to_localization_files {
                todo!()
            }
            if self.config.error_on_missing_localization_on_load {
                todo!()
            }
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
