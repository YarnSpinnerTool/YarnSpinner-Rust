use crate::prelude::*;
use bevy::asset::io::Reader;
use bevy::prelude::*;
use bevy::reflect::TypePath;

use anyhow::Result;
use bevy::asset::{AssetLoader, LoadContext};
use std::hash::Hash;
#[cfg(feature = "compiler")]
use yarnspinner::prelude::YarnFile as InnerYarnFile;

#[cfg(feature = "compiler")]
/// A Yarn file. These will mostly be created by loading them from disk with the [`AssetServer`].
#[derive(Debug, Clone, Eq, PartialEq, Reflect, Asset, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct YarnFile {
    pub(crate) file: InnerYarnFile,
    pub(crate) string_table: std::collections::HashMap<LineId, StringInfo>,
}

#[cfg(feature = "compiler")]
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

#[cfg(feature = "compiler")]
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

#[cfg(feature = "compiler")]
impl Hash for YarnFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.file.file_name.hash(state);
        self.file.source.hash(state);
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.init_asset::<YarnFile>()
        .register_asset_reflect::<YarnFile>()
        .init_asset_loader::<YarnFileAssetLoader>();
}

#[derive(Debug, Default, TypePath)]
struct YarnFileAssetLoader;

impl AssetLoader for YarnFileAssetLoader {
    type Asset = YarnFile;
    type Settings = ();
    type Error = anyhow::Error;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let yarn_file = read_yarn_file(bytes, load_context)?;
        Ok(yarn_file)
    }

    fn extensions(&self) -> &[&str] {
        &["yarn"]
    }
}

#[cfg(feature = "compiler")]
fn read_yarn_file(bytes: Vec<u8>, load_context: &LoadContext) -> Result<YarnFile, Error> {
    let source = String::from_utf8(bytes)?;
    let file_name = anyhow::Context::context(
        load_context.path().path().file_name(),
        "Yarn file has no filename",
    )?;
    let file_name =
        anyhow::Context::context(file_name.to_str(), "Yarn file name is not valid UTF-8")?
            .to_owned();
    let file = InnerYarnFile { file_name, source };
    let string_table = YarnCompiler::new()
        .with_compilation_type(CompilationType::StringsOnly)
        .add_file(file.clone())
        .compile()?
        .string_table;
    Ok(YarnFile { file, string_table })
}

/// A Yarn file. These will mostly be created by loading them from disk with the [`AssetServer`].
///
/// Without the `compiler` feature this only holds the file's name and source;
/// compilation happens externally via `ysc`.
#[cfg(not(feature = "compiler"))]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Asset, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct YarnFile {
    file_name: String,
    source: String,
}

#[cfg(not(feature = "compiler"))]
impl YarnFile {
    /// Creates a new Yarn file from a filename and file content.
    pub fn new(filename: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            file_name: filename.into(),
            source: content.into(),
        }
    }

    /// Returns the filename of the Yarn file.
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Returns the content of the Yarn file.
    pub fn content(&self) -> &str {
        &self.source
    }

    /// Overrides the content of the Yarn file. Note that for Yarn files loaded with an [`AssetServer`], this will *not* change the file on disk.
    pub fn set_content(&mut self, content: String) -> Result<&mut Self> {
        self.source = content;
        Ok(self)
    }
}

#[cfg(not(feature = "compiler"))]
fn read_yarn_file(bytes: Vec<u8>, load_context: &LoadContext) -> Result<YarnFile, Error> {
    // Note: `.context()` is ambiguous here because bevy's prelude exports `ContextExt`.
    let source = String::from_utf8(bytes)?;
    let file_name = load_context
        .path()
        .path()
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Yarn file name is missing or not valid UTF-8"))?
        .to_owned();
    Ok(YarnFile { file_name, source })
}
