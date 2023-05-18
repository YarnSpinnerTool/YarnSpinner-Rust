use crate::prelude::*;
use bevy::asset::LoadedAsset;
use bevy::prelude::*;
use bevy::{
    asset::{AssetLoader, LoadContext},
    utils::BoxedFuture,
};

#[derive(Debug)]
pub(crate) struct YarnSlingerAssetLoaderPlugin;
impl Plugin for YarnSlingerAssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<YarnSlingerLocalizationConfig>()
            .register_type::<YarnSlingerDefaultLocaleConfig>()
            .add_asset::<YarnFile>()
            .init_resource::<YarnSlingerLocalizationConfig>()
            .init_asset_loader::<YarnFileAssetLoader>()
            .init_resource::<YarnSlingerLocalizationConfig>()
            .add_system(generate_missing_line_ids_in_yarn_file.pipe(yarn_plugin_panic));
    }
}

#[derive(Debug, Default)]
struct YarnFileAssetLoader {
    config: YarnSlingerLocalizationConfig,
}

impl AssetLoader for YarnFileAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, SystemResult> {
        Box::pin(async move {
            let mut yarn_file = read_yarn_file(bytes, load_context)?;
            if self.config.generate_missing_line_ids_in_yarn_file && can_access_fs() {
                if let Some(source_with_added_ids) = add_tags_to_lines(yarn_file.clone())? {
                    std::fs::write(load_context.path(), &source_with_added_ids)
                        .context("Failed to write Yarn file with new line IDs")?;
                    yarn_file.source = source_with_added_ids;
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

fn generate_missing_line_ids_in_yarn_file(
    mut events: EventReader<AssetEvent<YarnFile>>,
    mut assets: ResMut<Assets<YarnFile>>,
    asset_server: Res<AssetServer>,
) -> SystemResult {
    for event in events.iter() {
        if let AssetEvent::Created { handle } = event {
            let file = assets.get_mut(handle).unwrap();
            if let Some(source_with_added_ids) = add_tags_to_lines(file.clone())? {
                let asset_path = asset_server.get_handle_path(handle.clone()).unwrap();
                std::fs::write(asset_path.path(), &source_with_added_ids).unwrap_or_else(|e| {
                    error!("Failed to write Yarn file with new line IDs: {e}");
                });
                file.source = source_with_added_ids;
            }
        }
    }
    Ok(())
}

const fn can_access_fs() -> bool {
    cfg!(not(target_arch = "wasm32"))
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
