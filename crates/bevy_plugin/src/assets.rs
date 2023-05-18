use crate::config::YarnSlingerLocalizationConfig;
use anyhow::Context;
use bevy::asset::LoadedAsset;
use bevy::{
    asset::{AssetLoader, Error, LoadContext},
    utils::BoxedFuture,
};
use yarn_slinger::prelude::*;

#[derive(Debug, Default)]
pub struct YarnFileAssetLoader {
    config: YarnSlingerLocalizationConfig,
}

impl AssetLoader for YarnFileAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let mut yarn_file = read_yarn_file(bytes, load_context)?;
            if self.config.generate_missing_line_ids_in_yarn_file {
                if let Some(content_with_ids) =
                    YarnCompiler::add_tags_to_lines(yarn_file.source.clone(), Vec::new())?
                {
                    std::fs::write(load_context.path(), &content_with_ids)
                        .context("Failed to write Yarn file with new line IDs")?;
                    yarn_file.source = content_with_ids;
                }
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
