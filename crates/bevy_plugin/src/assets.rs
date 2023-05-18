use crate::prelude::*;
use bevy::asset::LoadedAsset;
use bevy::prelude::*;
use bevy::{
    asset::{AssetLoader, LoadContext},
    utils::BoxedFuture,
};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub(crate) struct YarnSlingerAssetLoaderPlugin;

impl Plugin for YarnSlingerAssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Localizations>()
            .register_type::<Localization>()
            .register_type::<FileGenerationMode>()
            .add_asset::<YarnFile>()
            .init_asset_loader::<YarnFileAssetLoader>()
            .add_system(
                generate_missing_line_ids_in_yarn_file
                    .pipe(panic_on_err)
                    .run_if(is_in_development),
            );
    }
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

fn generate_missing_line_ids_in_yarn_file(
    mut events: EventReader<AssetEvent<YarnFile>>,
    mut assets: ResMut<Assets<YarnFile>>,
    asset_server: Res<AssetServer>,
) -> SystemResult {
    for event in events.iter() {
        if let AssetEvent::Created { handle } = event {
            let file = assets.get_mut(handle).unwrap();
            let Some(source_with_added_ids) = add_tags_to_lines(file.clone())? else {
                // File already contains all line IDs.
                return Ok(())
            };

            // If this fails, the asset was created at runtime and doesn't exist on disk.
            if let Some(asset_path) = asset_server.get_handle_path(handle.clone()) {
                let path_within_asset_dir: PathBuf =
                    [get_assets_dir_name().as_ref(), asset_path.path()]
                        .iter()
                        .collect();

                std::fs::write(&path_within_asset_dir, &source_with_added_ids).map_err(|e|
                        Error::new(e).context(format!("Failed to overwrite Yarn file at {} with new line IDs.\n\
                                Aborting because localization requires all lines to have IDs, but this file is missing some.", path_within_asset_dir.display()))
                    )?;
            }
            file.source = source_with_added_ids;
        }
    }
    Ok(())
}

fn get_assets_dir_name() -> impl AsRef<Path> {
    // This could be customized, but AFAIK there's no way to get that info
    AssetPlugin::default().asset_folder
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

/// Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Console/blob/main/src/YarnSpinner.Console/Commands/TagCommand.cs#L11>
fn add_tags_to_lines(yarn_file: YarnFile) -> YarnCompilerResult<Option<String>> {
    let existing_tags = YarnCompiler::new()
        .with_compilation_type(CompilationType::StringsOnly)
        .add_file(yarn_file.clone())
        .compile()
        .unwrap()
        .string_table
        .into_iter()
        .filter_map(|(key, string_info)| (!string_info.is_implicit_tag).then_some(key))
        .collect();
    YarnCompiler::add_tags_to_lines(yarn_file.source, existing_tags)
}
