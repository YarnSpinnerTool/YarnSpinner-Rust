use crate::prelude::*;
use bevy::asset::LoadedAsset;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::{
    asset::{AssetLoader, LoadContext},
    utils::BoxedFuture,
};
use yarn_slinger::prelude::YarnFile as InnerYarnFile;

#[derive(Debug, Clone, Eq, PartialEq, Reflect, FromReflect, TypeUuid, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Serialize, Deserialize)]
#[uuid = "32570e61-d69d-4f87-9552-9da2a62ecfd1"]
pub struct YarnFile {
    pub file: InnerYarnFile,
    pub string_table: std::collections::HashMap<LineId, StringInfo>,
}

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
    let file = InnerYarnFile { file_name, source };
    let string_table = YarnCompiler::new()
        .with_compilation_type(CompilationType::StringsOnly)
        .add_file(file.clone())
        .compile()
        .unwrap()
        .string_table;
    Ok(YarnFile { file, string_table })
}