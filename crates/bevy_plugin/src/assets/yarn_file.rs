use crate::prelude::*;
use bevy::asset::io::Reader;
use bevy::prelude::*;
use bevy::reflect::TypePath;

use anyhow::Result;
use bevy::asset::{AssetLoader, LoadContext};
/// A Yarn file. These will mostly be created by loading them from disk with the [`AssetServer`].
///
/// This only holds the file's name and source; compilation happens externally via `ysc`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Asset, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct YarnFile {
    file_name: String,
    source: String,
}

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
