use crate::prelude::*;
use bevy::prelude::*;
use std::path::PathBuf;

pub(crate) fn line_id_generation_plugin(app: &mut App) {
    app.add_system(
        generate_missing_line_ids_in_yarn_file
            .pipe(panic_on_err)
            .run_if(is_in_development),
    );
}

fn generate_missing_line_ids_in_yarn_file(
    mut events: EventReader<AssetEvent<YarnFile>>,
    mut assets: ResMut<Assets<YarnFile>>,
    asset_server: Res<AssetServer>,
) -> SystemResult {
    for event in events.iter() {
        if let AssetEvent::Created { handle } = event {
            let yarn_file = assets.get_mut(handle).unwrap();
            let Some(source_with_added_ids) = add_tags_to_lines(yarn_file.clone())? else {
                // File already contains all line IDs.
                return Ok(())
            };

            // If this fails, the asset was created at runtime and doesn't exist on disk.
            if let Some(asset_path) = asset_server.get_handle_path(handle.clone()) {
                let assets_path = get_assets_dir_path(&asset_server)?;
                let path_within_asset_dir: PathBuf =
                    [assets_path.as_ref(), asset_path.path()].iter().collect();

                std::fs::write(&path_within_asset_dir, &source_with_added_ids)
                    .context(
                        format!("Failed to overwrite Yarn file at {} with new line IDs. \
                                 Aborting because localization requires all lines to have IDs, but this file is missing some.",
                                path_within_asset_dir.display()))?;
            }
            yarn_file.file.source = source_with_added_ids;
        }
    }
    Ok(())
}

/// Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Console/blob/main/src/YarnSpinner.Console/Commands/TagCommand.cs#L11>
fn add_tags_to_lines(yarn_file: YarnFile) -> YarnCompilerResult<Option<String>> {
    let existing_tags = yarn_file
        .string_table
        .into_iter()
        .filter_map(|(key, string_info)| (!string_info.is_implicit_tag).then(|| key.clone()))
        .collect();
    YarnCompiler::add_tags_to_lines(yarn_file.file.source, existing_tags)
}
