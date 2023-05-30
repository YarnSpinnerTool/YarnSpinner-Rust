use crate::prelude::*;
use bevy::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum YarnFileSource {
    Handle(Handle<YarnFile>),
    InMemory(YarnFile),
    Path(PathBuf),
}

impl From<Handle<YarnFile>> for YarnFileSource {
    fn from(handle: Handle<YarnFile>) -> Self {
        Self::Handle(handle)
    }
}

impl From<String> for YarnFileSource {
    fn from(path: String) -> Self {
        Self::Path(path.into())
    }
}

impl From<&str> for YarnFileSource {
    fn from(path: &str) -> Self {
        Self::Path(path.into())
    }
}

impl From<PathBuf> for YarnFileSource {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl From<&Path> for YarnFileSource {
    fn from(path: &Path) -> Self {
        Self::Path(path.into())
    }
}

impl From<YarnFile> for YarnFileSource {
    fn from(yarn_file: YarnFile) -> Self {
        Self::InMemory(yarn_file)
    }
}

impl YarnFileSource {
    pub fn load(
        &self,
        asset_server: &AssetServer,
        assets: &mut Assets<YarnFile>,
    ) -> Handle<YarnFile> {
        match self {
            Self::Handle(handle) => handle.clone(),
            Self::InMemory(yarn_file) => assets.add(yarn_file.clone()),
            Self::Path(path) => asset_server.load(path.as_path()),
        }
    }
}
