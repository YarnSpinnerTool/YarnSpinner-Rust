use crate::prelude::*;
use bevy::asset::LoadedAsset;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::{
    asset::{AssetLoader, LoadContext},
    utils::BoxedFuture,
};
use std::hash::Hash;
use yarn_slinger::prelude::YarnFile as InnerYarnFile;

/// A Yarn file. These will mostly be created by loading them from disk with the [`AssetServer`].
#[derive(Debug, Clone, Eq, PartialEq, Reflect, FromReflect, TypeUuid, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
#[uuid = "32570e61-d69d-4f87-9552-9da2a62ecfd1"]
pub struct YarnFile {
    pub(crate) file: InnerYarnFile,
    pub(crate) string_table: std::collections::HashMap<LineId, StringInfo>,
}

impl YarnFile {
    /// Creates a new Yarn file from a filename and file content.
    pub fn new(filename: impl Into<String>, content: impl Into<String>) -> Self {
        let filename = filename.into();
        let content = content.into();
        let file = InnerYarnFile {
            file_name: filename,
            source: content,
        };
        let string_table = compile_string_table(file.clone()).unwrap();
        Self { file, string_table }
    }

    /// Returns the filename of the Yarn file.
    pub fn file_name(&self) -> &str {
        &self.file.file_name
    }

    /// Returns the content of the Yarn file.
    pub fn content(&self) -> &str {
        &self.file.source
    }

    /// Overrides the content of the Yarn file. Note that for Yarn files loaded with an [`AssetServer`], this will *not* change the file on disk.
    pub fn set_content(&mut self, content: String) -> Result<&mut Self> {
        self.file.source = content;
        let string_table = compile_string_table(self.file.clone())?;
        self.string_table = string_table;
        Ok(self)
    }
}

fn compile_string_table(
    file: InnerYarnFile,
) -> Result<std::collections::HashMap<LineId, StringInfo>> {
    let string_table = YarnCompiler::new()
        .with_compilation_type(CompilationType::StringsOnly)
        .add_file(file)
        .compile()?
        .string_table;
    Ok(string_table)
}

impl Hash for YarnFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.file.file_name.hash(state);
        self.file.source.hash(state);
    }
}

pub(crate) fn yarn_slinger_asset_loader_plugin(app: &mut App) {
    app.add_asset::<YarnFile>()
        .register_asset_reflect::<YarnFile>()
        .init_asset_loader::<YarnFileAssetLoader>();
}

#[derive(Debug, Default)]
struct YarnFileAssetLoader;

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
        .compile()?
        .string_table;
    Ok(YarnFile { file, string_table })
}
