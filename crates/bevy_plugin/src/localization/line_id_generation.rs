use crate::prelude::*;
use bevy::asset::FileAssetIo;
use bevy::prelude::*;
use std::path::{Path, PathBuf};

pub(crate) fn generate_missing_line_ids_in_yarn_file(
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
                let assets_path = get_assets_dir_name(&asset_server)?;
                let path_within_asset_dir: PathBuf =
                    [assets_path.as_ref(), asset_path.path()].iter().collect();

                std::fs::write(&path_within_asset_dir, &source_with_added_ids)
                    .context(
                        format!("Failed to overwrite Yarn file at {} with new line IDs. \
                                 Aborting because localization requires all lines to have IDs, but this file is missing some.",
                                path_within_asset_dir.display()))?;
            }
            file.source = source_with_added_ids;
        }
    }
    Ok(())
}

fn get_assets_dir_name(asset_server: &AssetServer) -> Result<impl AsRef<Path> + '_> {
    let asset_io = asset_server.asset_io();
    let file_asset_io = asset_io.downcast_ref::<FileAssetIo>().context(
        "Failed to downcast asset server IO to `FileAssetIo`. \
    The vanilla Bevy `FileAssetIo` is the only one supported by Yarn Slinger",
    )?;
    Ok(file_asset_io.root_path())
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
